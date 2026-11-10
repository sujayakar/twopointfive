use super::*;                      // your own module: its types, its other parts' functions
use crate::ask::Over;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// goTo's own planner: the floor's way as this man's feet will find it, at his prices when there is a man.
fn find_way(g: &mut Game, a: V3, b: V3, nodes: f64, who: Option<GuardId>) -> Option<Vec<V3>> {
    let pen = match who {
        Some(w) => { let t = crate::tactics::tactics_penalty(g, w, b); crate::lanes::lane_penalty(g, w, t.as_ref()) }
        None => None,
    };
    match pen {
        Some(l) => crate::scene_collision::nav_grid_find_path(g, a, b, Some(nodes), Some(&mut |_g: &Game, ix: f64, iz: f64| crate::lanes::lane_penalty_at(&l, ix, iz))),
        None => crate::scene_collision::nav_grid_find_path(g, a, b, Some(nodes), None),
    }
}

// ---------------------------------------------------------------- the burst
/// what blastRaw works out once, for its local closures (judgeAt, judge, away, finish)
struct Burst {
    at: V3,
    from: V3,
    lethal: f64,
    wound: f64,
    serial: f64,
    who: f64,
    side: crate::teams::Team,
    mine: bool,
    name: String,
    before: Option<JsMap<crate::Num, f64>>,
    torn: Vec<BoxId>,
}
/// the `shows` closure of judgeAt: what of him the burst has a line to now
#[derive(Clone, Copy)]
enum Shows<'a> {
    Man { feet: V3, head: V3 },
    Lying(&'a [V3]),
}
fn shows_now(g: &mut Game, over: &Over, from: V3, s: Shows) -> f64 {
    match s {
        Shows::Man { feet, head } => exposure_over(g, over, from, feet, head, Some(GlassView::Blown)),
        Shows::Lying(pts) => lying_shows_over(g, over, from, pts, GlassView::Blown),
    }
}
struct Judged {
    d: f64,
    e: f64,
    r: Option<BlastResult>,
    behind: Option<BlastHitBehind>,
}
fn judge_at(g: &mut Game, b: &Burst, id: f64, d: f64, shows: Shows) -> Judged {
    if d > b.wound { return Judged { d, e: 0.0, r: None, behind: None }; }
    let was = match &b.before { Some(m) => m.get(crate::Num(id)).copied(), None => None };
    let now = match was { Some(w) if !(w <= 0.0) => w, _ => shows_now(g, &Over::OWN, b.from, shows) };
    let e = was.unwrap_or(now);
    if e <= 0.0 {
        if now > 0.0 { return Judged { d, e: now, r: Some(BlastResult::Stunned), behind: Some(BlastHitBehind::Cover) }; }
        let from = b.from;
        let through = if !b.torn.is_empty() { shows_now(g, &Over::but(&b.torn), from, shows) } else { 0.0 };
        return if through > 0.0 { Judged { d, e: through, r: Some(BlastResult::Stunned), behind: Some(BlastHitBehind::Door) } } else { Judged { d, e, r: Some(BlastResult::Shielded), behind: None } };
    }
    if d <= b.lethal { return Judged { d, e, r: Some(if e >= 0.5 { BlastResult::Killed } else { BlastResult::Wounded }), behind: None }; }
    let t = (d - b.lethal) / m::max(1e-3, b.wound - b.lethal);
    let p = g.k.frag.wound_kill * (1.0 - t) * (1.0 - t) * (if e >= 0.5 { 1.0 } else { 0.4 });
    Judged { d, e, r: Some(if hash01(b.serial, id) < p { BlastResult::Killed } else { BlastResult::Wounded }), behind: None }
}
fn judge(g: &mut Game, b: &Burst, feet: V3, head: V3, id: f64) -> Judged {
    let d = v3::dist(b.from, [feet[0], feet[1] + 0.9, feet[2]]);
    judge_at(g, b, id, d, Shows::Man { feet, head })
}
fn away(at: V3, feet: V3) -> V3 {
    let d: V3 = [feet[0] - at[0], 0.0, feet[2] - at[2]];
    let l = m::hyp2(d[0], d[2]);
    if l > 1e-3 { v3::normalize([d[0] / l, 0.45, d[2] / l]) } else { [0.0, 1.0, 0.0] }
}
/// `char.bones.<bone> ?? char.pos + [0, up, 0]`
fn bone_or(g: &Game, c: CharId, bone: crate::character::Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(bone) { Some(p) => *p, None => v3::add(g.ch[c].pos, [0.0, up, 0.0]) }
}
/// who the kill is down to, as blastRaw says it
fn down_by(b: &Burst) -> crate::game::DownBy {
    if b.mine { crate::game::DownBy::Player } else if js::truthy(b.who) { crate::game::DownBy::Guard } else { crate::game::DownBy::World }
}
/// A man already down and breathing: judged where he lies, killed or left exactly as he was.
fn finish(g: &mut Game, b: &Burst, rep: &mut BlastReport, gd: GuardId) {
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    let pts = lying(g, gd);
    let d = v3::dist(b.from, pts[0]);
    let j = judge_at(g, b, id, d, Shows::Lying(&pts));
    if j.r != Some(BlastResult::Killed) && !(j.r == Some(BlastResult::Wounded) && j.d <= b.lethal) { return; }
    let (callsign, team) = (g.gd[gd].callsign.clone(), g.team(gd));
    rep.hits.push(BlastHit { id, team, exposed: j.e, result: BlastResult::Killed });
    rep.killed += 1.0;
    rep.finished += 1.0;
    if b.side == team && js::truthy(b.who) { g.stats.frag.own += 1; }
    let dir = away(b.at, pts[0]);
    let fresh = g.gd[gd].died_at == g.time;
    let had = g.gd[gd].down_by == Some(crate::game::DownBy::Player) || (!fresh && crate::teamtally::team_tallied(g, gd));
    g.gd[gd].down_kind = Some(crate::game::DownKind::Shot);
    g.gd[gd].down_by = Some(down_by(b));
    if js::truthy(b.who) { g.gd[gd].killer_id = b.who; }
    let team_ = !b.mine && crate::teamtally::team_tallied(g, gd);
    if crate::mission::mission_live(g) && (b.mine || team_) {
        if had && g.mission.stats.knockouts > 0.0 { g.mission.stats.knockouts -= 1.0; }
        if b.mine || !fresh { g.mission.stats.kills += 1.0; }
    }
    if b.mine || (team_ && !fresh) {
        let weapon = if b.mine { alloc::format!("the {}", b.name) } else { alloc::format!("a teammate's {}", b.name) };
        let mut data: JsMap<String, Option<crate::chronicle::ChronicleDataValue>> = JsMap::new();
        data.set(String::from("who"), Some(crate::chronicle::ChronicleDataValue::Text(callsign.clone())));
        data.set(String::from("weapon"), Some(crate::chronicle::ChronicleDataValue::Text(weapon)));
        data.set(String::from("blast"), Some(crate::chronicle::ChronicleDataValue::Bool(true)));
        crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Kill, Some(&data), Some(pts[0]));
    }
    if b.mine { crate::guards::witness_kill(g, gd, crate::game::DownKind::Shot, crate::game::DownBy::Player); }
    if g.ch[c].ragdoll.is_none() {
        crate::character::character_ragdollize(g, c, true, dir, true);
    }
    let shove = v3::scale(dir, g.k.frag.down_shove * m::max(0.0, 1.0 - j.d / b.wound));
    if let Some(r) = g.ch[c].ragdoll { crate::ragdoll::ragdoll_add_rigid_velocity(g, r, shove, [0.0, 0.0, 0.0]); }
}

/// One seat's man and the burst.
fn seat_blast(g: &mut Game, b: &Burst, rep: &mut BlastReport) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if crate::condition::down(g, g.pl[pl].ch) { return; }
    let head = bone_or(g, c, crate::character::Bone::Head, 1.6);
    let (feet, id) = (g.ch[c].pos, g.ch[c].id);
    let j = judge(g, b, feet, head, id);
    let Some(r) = j.r else { return };
    rep.player = Some(r);
    rep.hits.push(BlastHit { id, team: crate::teams::Team::Ours, exposed: j.e, result: r });
    if r == BlastResult::Killed {
        if g.god_mode {
            rep.player = Some(BlastResult::Wounded);
        } else {
            let dir = away(b.at, g.ch[c].pos);
            crate::combat::hit_player(g, dir);
            g.pl[pl].hit_flash = 1.0;
            let mut by: Option<String> = None;
            for x in g.guards.clone() { if g.ch[g.gd[x].ch].id == b.who { by = Some(g.gd[x].callsign.clone()); break; } }
            let by = by.unwrap_or_else(|| if b.mine { String::from("his own grenade") } else { b.name.clone() });
            crate::chronicle::chron_hit(g, Some(&by));
        }
    }
    if rep.player == Some(BlastResult::Wounded) { crate::condition::condition_hurt(g, c); }
    if rep.player == Some(BlastResult::Wounded) || rep.player == Some(BlastResult::Stunned) {
        let st = g.k.frag.stagger;
        let secs = v3::lerp(st[0], st[1], v3::clamp(j.d / b.wound, 0.0, 1.0));
        let along = away(b.at, g.ch[c].pos);
        crate::samstun::stagger_player(g, secs, along);
        crate::deed::shaken_off(g, pl);
        g.pl[pl].pending_throw = None;
    }
}

/// The burst itself, in its order: flash and sound, glass, lines before the break, destruction, torn leaves, the men, the books.
pub(super) fn blast_raw(g: &mut crate::game::Game, at: V3, spec: &crate::frag::BlastSpec) -> crate::frag::BlastReport {
    let lethal = spec.lethal_m.unwrap_or(g.k.frag.lethal_m);
    let wound = m::max(lethal, spec.wound_m.unwrap_or(g.k.frag.wound_m));
    let serial = match spec.serial { Some(s) => s, None => m::floor(g.time * 1000.0) };
    let from: V3 = [at[0], at[1] + g.k.frag.lift, at[2]];
    let name: String = match &spec.name { Some(n) => n.clone(), None => String::from("frag grenade") };
    let mut rep = BlastReport { at: v3::copy(at), hits: Vec::new(), killed: 0.0, finished: 0.0, player: None };
    let side = if js::truthy(spec.who) { crate::teams::team_of_id(g, spec.who) } else { crate::teams::Team::Guard };
    let mine = crate::seats::is_man_id(g, spec.who);
    if spec.fx != Some(false) {
        let seed = g.fx_seed;
        g.fx_seed += 1.0;
        let (bang, flags) = (crate::effects::bang(), crate::effects::flags());
        let o = crate::effects::SpawnBurstO { spark_count: Some(m::min(bang.spark_count, if flags.trails { bang.trail_count } else { 0.0 })) };
        let l = crate::effects::spawn_burst(g, from, seed, Some(o));
        crate::particle_fx::bang_fx(g, from);
        let flash = crate::render_lights::light_set_flash(g, l.pos, [1.0, 0.74, 0.42], l.intensity * 0.7, l.range, l.ttl, 0.0);
        let light = crate::fogofwar::fog_of_war_light_by(g, flash, Some(spec.who));
        if let Some(r) = l.radius { if js::truthy(r) { g.tab.light[light].radius = r; } }
        crate::audio_iface::game_audio_like_play(g, "stunBang", Some(from), Some(1.0), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.62 }));
        crate::marks::bang_scorch(g, from);
        let ears = crate::seats::host_man(g);
        let eye = bone_or(g, g.pl[ears].ch, crate::character::Bone::Head, 1.6);
        if !crate::condition::down(g, g.pl[ears].ch) && v3::dist(eye, from) < wound && !crate::scene_collision::segment_blocked(g, from, eye, None, None, None) {
            crate::audio_iface::game_audio_like_play(g, "earRing", None, Some(v3::clamp(1.1 - v3::dist(eye, from) / wound, 0.2, 0.7)), None);
        }
    }
    // the glass first: nothing reaches it after the men are judged through it
    if g.k.frag.glass_power > 0.0 {
        let o = crate::glassbreak::GlassBlastOpts { radius: Some(g.k.frag.glass_m), power: Some(g.k.frag.glass_power), who: Some(spec.who) };
        crate::glassbreak::glass_break_blast(g, from, Some(o));
    }
    crate::glassbreak::glass_break_bang(g, [from[0], m::max(0.3, from[1]), from[2]], spec.who);
    let list = g.doors.list.clone();
    let mut hung: Vec<DoorId> = Vec::new();
    for d in list { if !g.dr[d].leaf.off { hung.push(d); } }
    // what the burst had a line to before it broke anything
    let mut before: Option<JsMap<crate::Num, f64>> = None;
    if crate::destruction::chaos_of(g) > 0.0 {
        let mut map: JsMap<crate::Num, f64> = JsMap::new();
        let mut all = g.guards.clone();
        all.extend(crate::civilians::civilians_shown(g));
        for gd in all {
            if !crate::condition::down(g, g.gd[gd].ch) && !g.gd[gd].god {
                let c = g.gd[gd].ch;
                let (feet, head, id) = (g.ch[c].pos, bone_or(g, c, crate::character::Bone::Head, 1.65), g.ch[c].id);
                if v3::dist(from, [feet[0], feet[1] + 0.9, feet[2]]) <= wound { let e = exposure(g, from, feet, head, Some(GlassView::Blown)); map.set(crate::Num(id), e); }
            }
        }
        for gd in g.guards.clone() {
            if down_asked(g, gd) {
                let pts = lying(g, gd);
                if v3::dist(from, pts[0]) <= wound { let e = lying_shows(g, from, &pts, GlassView::Blown); let id = g.ch[g.gd[gd].ch].id; map.set(crate::Num(id), e); }
            }
        }
        for pl in crate::seats::men(g) {
            if !crate::condition::down(g, g.pl[pl].ch) {
                let c = g.pl[pl].ch;
                let (feet, head, id) = (g.ch[c].pos, bone_or(g, c, crate::character::Bone::Head, 1.6), g.ch[c].id);
                if v3::dist(from, [feet[0], feet[1] + 0.9, feet[2]]) <= wound { let e = exposure(g, from, feet, head, Some(GlassView::Blown)); map.set(crate::Num(id), e); }
            }
        }
        before = Some(map);
    }
    let (radius, power) = match spec.destr { Some(d) => (d.radius, d.power), None => (None, None) };
    crate::destruction::destruction_blast(g, from, Some(crate::wallbreak::BlastOpts { who: Some(spec.who), radius, power, ..Default::default() }));
    // the leaves this burst (and this tick's others) took off: asked again with them out
    let fresh_book = match &g.m.frag.torn { Some(t) => t.time != g.time, None => true };
    if fresh_book { g.m.frag.torn = Some(FragModTORNValue { time: g.time, doors: Vec::new() }); }
    for d in hung {
        if g.dr[d].leaf.off {
            let t = g.m.frag.torn.as_mut().unwrap();
            if !t.doors.contains(&d) { t.doors.push(d); }
        }
    }
    let rows: Vec<DoorId> = g.m.frag.torn.as_ref().unwrap().doors.clone();
    let mut torn: Vec<BoxId> = Vec::new();
    for d in rows { if g.dr[d].leaf.off { torn.push(g.dr[d].box_); } }
    let ev_serial = g.serial();
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Bang, pos: v3::copy(from), time: g.time, loud: true, level: Some(2.0), who: Some(spec.who), frag: Some(true), serial: ev_serial, ..Default::default() });
    let b = Burst { at, from, lethal, wound, serial, who: spec.who, side, mine, name, before, torn };
    // who it touches
    let mut all = g.guards.clone();
    all.extend(crate::civilians::civilians_shown(g));
    for gd in all {
        if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].god {
            if down_asked(g, gd) { finish(g, &b, &mut rep, gd); }
            continue;
        }
        let c = g.gd[gd].ch;
        let (pos, head, id) = (g.ch[c].pos, bone_or(g, c, crate::character::Bone::Head, 1.65), g.ch[c].id);
        let j = judge(g, &b, pos, head, id);
        let Some(r) = j.r else { continue };
        let team = g.team(gd);
        rep.hits.push(BlastHit { id, team, exposed: j.e, result: r });
        if r == BlastResult::Shielded {  continue; }
        if side == team && js::truthy(spec.who) { g.stats.frag.own += 1; }
        if r == BlastResult::Killed {
            rep.killed += 1.0;
            let dir = away(at, g.ch[c].pos);
            // (on the thrower's seat, the debrief's row naming what went off)
            let thrower = crate::seats::seat_of_id(g, spec.who);
            crate::seats::on_seat(g, thrower, &mut |g| crate::combat::kill_guard(g, gd, dir, Some(false), Some(crate::game::DownKind::Shot), Some(down_by(&b)), crate::game::With::Blast(&b.name)));
            if js::truthy(spec.who) { g.gd[gd].killer_id = spec.who; }
        } else {
            if g.gd[gd].civ.is_none() {
                let u = v3::clamp(j.d / wound, 0.0, 1.0);
                let st = g.k.frag.stagger;
                let secs = v3::lerp(st[0], st[1], u);
                let w = g.gd[gd].weapon;
                crate::condition::fold(g, c, secs);
                g.fa[w].burst = None;
                crate::condition::dazed(g, gd, secs, at, 0.92, "frag");
                if r == BlastResult::Wounded { crate::condition::condition_hurt(g, c); }
            }
            crate::character::anim_hit(g, c);
        }
    }
    crate::seats::each_seat(g, &mut |g: &mut Game| seat_blast(g, &b, &mut rep));
    crate::morale::morale_blast(g, at, Some(spec.who));
    g.stats.frag.bursts += 1;
    g.m.frag.frag_log.push(rep.clone());
    if g.m.frag.frag_log.len() > 64 { g.m.frag.frag_log.remove(0); }
    rep
}

/// The hand grenade goes off where it lies, and the item is gone.
fn detonate_frag(g: &mut crate::game::Game, it: ItemId) -> crate::frag::BlastReport {
    g.tab.item[it].fired = true;
    g.tab.item[it].life = 0.0;
    let at = g.tab.item[it].pos;
    let who = g.tab.item[it].who;
    let serial = g.tab.item[it].serial;
    let spec = BlastSpec { who, serial: Some(serial), name: Some(String::from("frag grenade")), destr: Some(BlastSpecDestr { radius: Some(g.k.frag.destr_m), power: Some(1.0) }), ..Default::default() };
    blast(g, at, &spec)
}

/// The fuse runs from the hand, in the air or on the floor.
pub fn frag_fuse(g: &mut crate::game::Game, it: ItemId, dt: f64) {
    if g.tab.item[it].fired { return; }
    let fuse = g.tab.item[it].fuse - dt;
    g.tab.item[it].fuse = fuse;
    if g.tab.item[it].fuse <= 0.0 { detonate_frag(g, it); }
}

/// The game's list of live grenades: the row itself, a writer writes through it.
pub(super) fn books_of(g: &mut crate::game::Game) -> &mut Vec<crate::frag::Live> {
    g.m.frag.books.get_or_insert_with(Vec::new)
}

/// Forget every live grenade's book: nobody is left running from something no longer there.
pub fn frag_reset(g: &mut crate::game::Game) {
    g.m.frag.def_books = None;
    let Some(books) = g.m.frag.books.as_ref() else { return };
    let men: Vec<GuardId> = books.iter().flat_map(|l| l.flee.keys()).collect();
    let (frag, time) = (crate::owner::by::FRAG, g.time);
    for gd in men {
        crate::game::guard_set_path(g, gd, Vec::new());
        g.gd[gd].path_goal = None;
        g.gd[gd].path_goal_orbit = false;
        crate::owner::release(g, gd, frag, time);
    }
    g.m.frag.books.as_mut().unwrap().clear();
}

/// what rollOn reads of the thing it flies: a live item, or a body that was never spawned (restOfThrow)
#[derive(Clone, Copy)]
struct RollBody {
    pos: V3,
    vel: V3,
    radius: f64,
    restitution: f64,
    friction: f64,
    bounces: f64,
    fuse: f64,
}
fn roll_slow(g: &Game, v: V3, bog: f64) -> bool {
    m::abs(v[1]) < 0.5 && m::hyp2(v[0], v[2]) < 0.35 + (if bog > 0.0 { crate::throwables::bog_stick(g, bog) } else { 0.0 })
}
/// The item step's own bounce and roll replayed on a copy for what is left of its fuse: nothing is moved or sounded.
fn roll_body(g: &Game, it: RollBody, touch_fuse: Option<f64>, props: bool, high: bool) -> RollOn {
    let (mut p, mut v) = (it.pos, it.vel);
    let h = 1.0 / 120.0;
    let mut path: Vec<V3> = Vec::new();
    let mut touched = it.bounces > 0.0;
    let mut last: Option<V3> = None;
    let mut bog: f64;
    let span = m::min(14.0, m::hyp2(v[0], v[2]) * m::max(0.0, it.fuse)) + 1.0;
    let mut furn = false;
    if props {
        for &pr in &g.props.props {
            let (x, z) = (g.tab.props_prop[pr].x, g.tab.props_prop[pr].z);
            if m::hyp2(x - p[0], z - p[2]) < span + g.tab.props_prop[pr].radius { furn = true; break; }
        }
    }
    let mut i = m::min(720.0, m::ceil(m::max(0.0, it.fuse) / h));
    while i > 0.0 {
        v[1] -= crate::throwables::GRAVITY * h;
        let mut done = false;
        let air = !touched;
        let mv = v3::scale(v, h);
        let len = v3::len(mv);
        if len < 1e-6 { i -= 1.0; continue; }
        let dir = v3::scale(mv, 1.0 / len);
        let tmax = len + it.radius * 3.5;
        // (t, normal, a prop's)
        let mut hit: Option<(f64, V3, bool)> = crate::scene_collision::raycast(g, p, dir, tmax).map(|x| (x.t, x.n, false));
        let mut back = 0.0;
        if furn {
            let upto = match hit { Some(x) => x.0, None => tmax };
            if let Some(x) = crate::scene_collision::things_ray(g, p, dir, upto) { hit = Some((x.t, x.n, true)); }
        }
        if let Some(x) = hit {
            back = it.radius / m::max(0.3, -v3::dot(dir, x.1));
            if x.0 - back > len { hit = None; }
        }
        if let Some((t, n, on_prop)) = hit {
            p = v3::mad(p, dir, m::max(0.0, t - back));
            let vn = v3::dot(v, n);
            if vn < 0.0 {
                let vn_ = v3::scale(n, vn);
                let vt = v3::sub(v, vn_);
                v = v3::add(v3::scale(vt, m::max(0.0, 1.0 - it.friction * (0.5 + 0.5 * m::abs(n[1])))), v3::scale(vn_, -it.restitution));
                bog = if n[1] > 0.6 && !on_prop { crate::snow_bog::snow_bog_at(g, p[0], p[1] - it.radius, p[2]) } else { 0.0 };
                if bog > 0.0 {
                    let up = v3::dot(v, n);
                    let (b, s) = (crate::throwables::bog_bounce(g, bog), crate::throwables::bog_slide(g, bog));
                    v = v3::add(v3::scale(v3::sub(v, v3::scale(n, up)), s), v3::scale(n, up * b));
                }
                touched = true;
                if n[1] > 0.6 && roll_slow(g, v, bog) { done = true; }
            }
            p = v3::mad(p, n, 0.002);
        } else {
            p = v3::add(p, mv);
        }
        if p[1] < it.radius {
            bog = crate::snow_bog::snow_bog_at(g, p[0], 0.0, p[2]);
            p[1] = it.radius;
            if v[1] < 0.0 {
                v[1] *= -it.restitution;
                v[0] *= 0.7;
                v[2] *= 0.7;
                if bog > 0.0 {
                    v[1] *= crate::throwables::bog_bounce(g, bog);
                    v[0] *= crate::throwables::bog_slide(g, bog);
                    v[2] *= crate::throwables::bog_slide(g, bog);
                }
                touched = true;
            }
            if roll_slow(g, v, bog) { done = true; }
        }
        let far = match last { None => true, Some(q) => flat(q, p) > 0.25 };
        if touched && far {
            let q: V3 = [p[0], if high { p[1] - it.radius } else { 0.0 }, p[2]];
            last = Some(q);
            path.push(q);
        }
        if air && touched { if let Some(tf) = touch_fuse { i = m::ceil(tf / h) + 1.0; } }
        if done { break; }
        i -= 1.0;
    }
    RollOn { at: [p[0], m::max(0.05, p[1]), p[2]], path }
}

/// Where a live thrown thing will rest and the floor it crosses to get there.
fn roll_on(g: &mut crate::game::Game, it: ItemId, touch_fuse: Option<f64>, props: Option<bool>, high: Option<bool>) -> crate::frag::RollOn {
    let body = RollBody {
        pos: g.tab.item[it].pos,
        vel: g.tab.item[it].vel,
        radius: g.tab.item[it].radius,
        restitution: g.tab.item[it].restitution,
        friction: g.tab.item[it].friction,
        bounces: g.tab.item[it].bounces,
        fuse: g.tab.item[it].fuse,
    };
    roll_body(g, body, touch_fuse, props.unwrap_or(true), high.unwrap_or(false))
}

/// Where a can thrown from `from` at `v0` will be when it is done moving, or when its fuse finds it.
pub fn rest_of_throw(g: &Game, kind: crate::game::ThrowKind, from: V3, v0: V3) -> crate::frag::RestOfThrow {
    let is_frag = kind == crate::game::ThrowKind::Frag;
    let (radius, restitution, friction) = if is_frag {
        let b = crate::throwables::frag_item();
        (b.radius, b.restitution, b.friction)
    } else {
        let b = crate::throwables::can_item();
        (b.radius, b.restitution, b.friction)
    };
    let stun = if kind == crate::game::ThrowKind::Flash { Some(crate::combat::flash_fuse(g)) } else { None };
    let fuse = if is_frag { g.k.frag.fuse } else if stun.is_some() { crate::effects::bang().max_air } else { 6.0 };
    let r = roll_body(g, RollBody { pos: from, vel: v0, radius, restitution, friction, bounces: 0.0, fuse }, stun, true, true);
    RestOfThrow { at: r.at, path: r.path }
}

/// What the thing he is aiming would reach, and round where (None: the rings off, no arc up, or a thing with no burst): the numbers the burst is judged by, round where a can
/// comes to rest (`rest`) or where the launcher's arc ends.
pub fn throw_reach(g: &crate::game::Game, pl: PlayerId, rest: Option<V3>) -> Option<crate::view::ThrowReachView> {
    use crate::game::ThrowKind;
    let sol = g.sets[crate::seats::seat_of_man(g, pl)].hand.throw_preview.as_ref().filter(|s| !s.points.is_empty())?;
    let burst = |at: V3| crate::view::ThrowReachView { at, kill: g.k.frag.lethal_m, hurt: m::max(g.k.frag.lethal_m, g.k.frag.wound_m), stun: 0.0 };
    if let Some(kind) = crate::player::can_in_hand(g, pl) {
        if kind != ThrowKind::Flash && !(kind == ThrowKind::Frag) { return None; }
        let at = rest?;
        return Some(if kind == ThrowKind::Frag { burst(at) } else { crate::view::ThrowReachView { at, kill: 0.0, hurt: 0.0, stun: crate::effects::bang().dazzle_radius } });
    }
    if crate::he40::he40_ready(g, pl) && crate::he40::arc_armed(g, sol) { Some(burst(sol.land)) } else { None }
}

/// The defenders' live grenades the man has been cued on: where each is, and the seconds left.
pub fn frag_marks(g: &crate::game::Game) -> Vec<crate::view::FragMark> {
    let Some(books) = &g.m.frag.books else { return Vec::new() };
    books.iter().filter(|l| l.cued && !g.tab.item[l.it].fired).map(|l| crate::view::FragMark { at: g.tab.item[l.it].pos, left: g.tab.item[l.it].fuse }).collect()
}

/// The height a grenade lying at y is judged from, less the lift: what it lies on.
fn rest_y(y: f64) -> f64 {
    m::max(0.05, y)
}

/// The rest reckoning of book `l` of the live grenades, made again ten times a second.
pub(super) fn rest_of(g: &mut crate::game::Game, l: usize) -> crate::frag::Rest {
    let it = books_of(g)[l].it;
    let settled = g.tab.item[it].settled;
    let time = g.time;
    if let Some(r) = &books_of(g)[l].rest {
        if r.still == settled && (settled || time - r.t < 0.1) { return r.clone(); }
    }
    let rest = if settled {
        let p = g.tab.item[it].pos;
        Rest { t: time, still: true, at: [p[0], rest_y(p[1]), p[2]], path: Vec::new() }
    } else {
        let r = roll_on(g, it, None, None, None);
        Rest { t: time, still: false, at: r.at, path: r.path }
    };
    books_of(g)[l].rest = Some(rest.clone());
    rest
}

/// Would a straight leg a → b shoulder a mate staying at q.
fn shoulders(g: &mut crate::game::Game, a: V3, b: V3, q: V3) -> bool {
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]);
    let l = dx * dx + dz * dz;
    let d0 = m::hyp2(q[0] - a[0], q[2] - a[2]) - 0.05;
    let (mut x, mut z) = (b[0], b[2]);
    if !(m::hyp2(x - q[0], z - q[2]) < m::min(g.k.frag.flee_apart_m, d0)) {
        let t = if l > 1e-6 { m::max(0.0, m::min(1.0, ((q[0] - a[0]) * dx + (q[2] - a[2]) * dz) / l)) } else { 0.0 };
        x = a[0] + dx * t;
        z = a[2] + dz * t;
        if !(m::hyp2(x - q[0], z - q[2]) < m::min(g.k.frag.flee_lane_m, d0)) { return false; }
    }
    !crate::scene_collision::solid_blocked(g, [x, 0.45, z], [q[0], 0.45, q[2]], None)
}

/// The leaves standing open within `r` m of `p`, each as hinge and end on the floor (x, z, x, z).
fn open_leaves(g: &crate::game::Game, p: V3, r: f64) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::new();
    for x in g.doors.list.clone() {
        if !g.dr[x].leaf.off && m::abs(g.dr[x].leaf.angle) > 0.15 && flat(g.dr[x].frame_centre, p) < r {
            let u = crate::doors::door_dir(g, x);
            let (h, w) = (g.dr[x].def.hinge, g.dr[x].def.width);
            out.push(vec![h[0], h[1], h[0] + u[0] * w, h[1] + u[1] * w]);
        }
    }
    out
}

/// How near (x, z) is to the nearest of those leaves.
fn leaf_dist(l: &[Vec<f64>], x: f64, z: f64) -> f64 {
    let mut d = f64::INFINITY;
    for s in l {
        let (ax, az, bx, bz) = (s[0], s[1], s[2], s[3]);
        let (ex, ez) = (bx - ax, bz - az);
        let t = m::max(0.0, m::min(1.0, ((x - ax) * ex + (z - az) * ez) / (ex * ex + ez * ez)));
        d = m::min(d, m::hyp2(x - ax - ex * t, z - az - ez * t));
    }
    d
}

/// A straight leg a → b comes within fleeDoorLeaf of a leaf, and nearer it than `ref_`.
fn leg_by_leaf(g: &crate::game::Game, l: &[Vec<f64>], a: V3, b: V3, ref_: f64) -> bool {
    let r = flat(a, b);
    let mut t = 0.25;
    while t < r + 0.25 {
        let u = if r > 1e-3 { m::min(1.0, t / r) } else { 1.0 };
        let d = leaf_dist(l, a[0] + (b[0] - a[0]) * u, a[2] + (b[2] - a[2]) * u);
        if d < g.k.frag.flee_door_leaf && d < ref_ - 0.05 { return true; }
        t += 0.25;
    }
    false
}

/// the rings of the second walk: between the first walk's
const LOOK_RINGS: [f64; 6] = [1.6, 2.5, 3.5, 4.5, 5.0, 6.25];
const FIRST_RINGS: [f64; 6] = [1.2, 2.0, 3.0, 4.0, 5.5, 7.0];

/// : a spot only a wall or leaf the burst will take hides
fn flee_shade(g: &mut Game, from: V3, p: V3) -> bool {
    exposure(g, from, p, [p[0], 1.15, p[2]], None) <= 0.0
}

/// Where a man in a grenade's line goes with `left` seconds on it; None: he drops where he is.
pub fn flee_spot(g: &mut crate::game::Game, c_2: crate::frag::FleeSpotC, at: V3, speed: f64, left: f64, accel: Option<f64>, wise: Option<&crate::frag::FleeSpotWise>, who: Option<GuardId>) -> Option<V3> { flee_spot_kept(g, c_2, at, speed, left, accel, wise, who).0 }

/// `fleeSpot`, and whether its answer is THE LEG HE HAS (`wise.keep` itself, the object: frag.ts:765 asks `spot === keep.goal`, an identity; a spot
/// weighed afresh with the kept goal's very numbers is another spot there). The body is fleeSpot's, whole.
pub(super) fn flee_spot_kept(g: &mut crate::game::Game, c_2: crate::frag::FleeSpotC, at: V3, speed: f64, left: f64, accel: Option<f64>, wise: Option<&crate::frag::FleeSpotWise>, who: Option<GuardId>) -> (Option<V3>, bool) {
    let accel = accel.unwrap_or(f64::INFINITY);
    let cpos = c_2.pos;
    let mut ask = match who { Some(w) => crate::picture::exposure_asker(g, w, Some(true)), None => None };
    let mind = ask.is_some();
    let (flee_cost, lethal_m, wound_m, flee_keep, flee_apart_m) = (g.k.picture.flee_cost, g.k.frag.lethal_m, g.k.frag.wound_m, g.k.frag.flee_keep, g.k.frag.flee_apart_m);
    let mut plain_s;
    let from: V3 = [at[0], at[1] + g.k.frag.lift, at[2]];
    let tt = m::max(0.0, left - 0.15);
    let ta = speed / accel;
    let reach = m::max(0.8, if tt <= ta { 0.5 * accel * tt * tt } else { speed * (tt - ta / 2.0) });
    let d0 = m::hyp2(cpos[0] - at[0], cpos[2] - at[2]);
    let a0 = m::atan2(cpos[0] - at[0], cpos[2] - at[2]);
    let mut best: Option<V3> = None;
    let mut best_s = d0 + 0.7;
    let mut bar: Option<V3> = None;
    let mut bar_s = f64::NEG_INFINITY;
    let mut mates = false;
    // identity of the spots, for `best !== plain` and `best !== wise.keep`: 0 none, 1 the leg he has, then one a spot weighed
    let (mut best_id, mut plain_id, mut next_id) = (0u32, 0u32, 1u32);
    let off_path = |x: f64, z: f64| -> f64 {
        let mut d = lethal_m;
        if let Some(w) = wise { for q in &w.path { d = m::min(d, m::hyp2(x - q[0], z - q[2])); } }
        d
    };
    let path0 = if wise.is_some() { off_path(cpos[0], cpos[2]) } else { 0.0 };
    let holds = |p: V3, hidden: bool| -> bool {
        let (dx, dz) = (p[0] - cpos[0], p[2] - cpos[2]);
        let r = m::hyp2(dx, dz);
        let dp = m::hyp2(p[0] - at[0], p[2] - at[2]);
        if if hidden { dp < d0 - 0.05 } else { dp <= d0 + 0.3 } { return false; }
        let mut s = 0.4;
        loop {
            let u = m::min(1.0, s / m::max(r, 1e-3));
            let (x, z) = (cpos[0] + dx * u, cpos[2] + dz * u);
            if m::hyp2(x - at[0], z - at[2]) < d0 - 0.2 || off_path(x, z) < path0 - 0.3 { return false; }
            if u >= 1.0 { return true; }
            s += 0.4;
        }
    };
    if wise.is_some() && d0 < lethal_m { best_s = f64::NEG_INFINITY; }
    plain_s = best_s;
    let score = |hidden: bool, shaded: bool, r: f64, dp: f64| -> f64 {
        if hidden { 100.0 - r * 2.0 + m::min(dp, 4.0) * 0.2 } else { (if shaded { 40.0 } else { 0.0 }) + m::min(dp, wound_m + 1.0) - 0.15 * r }
    };
    let mut best_open = false;
    if let Some(p) = wise.and_then(|w| w.keep) {
        let (r, dp) = (m::hyp2(p[0] - cpos[0], p[2] - cpos[2]), m::hyp2(p[0] - at[0], p[2] - at[2]));
        let hidden = exposed_to(g, from, p, [p[0], 1.15, p[2]], None) <= 0.0;
        let shaded = !hidden && flee_shade(g, from, p);
        if dp >= d0 - 0.05 && holds(p, true) {
            best = Some(p);
            best_id = 1;
            plain_id = 1;
            best_open = !hidden && !shaded;
            plain_s = score(hidden, shaded, r, dp) + flee_keep;
            // seenKept: only a firm line turns him off the leg he is on
            let kept = if hidden {
                let e = match ask.as_mut() { Some(f) => crate::picture::exposure_ask(g, f, p), None => 0.0 };
                if e >= 0.5 { flee_cost * e } else { 0.0 }
            } else { 0.0 };
            best_s = plain_s - kept;
        }
    }
    let leaves0 = if wise.is_some() { open_leaves(g, cpos, 8.0) } else { Vec::new() };
    let leaves = if !leaves0.is_empty() { Some(leaves0) } else { None };
    let leaf0 = match &leaves { Some(l) => leaf_dist(l, cpos[0], cpos[2]), None => 0.0 };
    let turns = if wise.is_some() { 32 } else { 16 };
    let step = m::PI / (if wise.is_some() { 16.0 } else { 8.0 });
    for look in 0..2 {
        if look != 0 && !(wise.is_some() && mates && (best.is_none() || best_open)) { break; }
        let rings: &[f64; 6] = if look != 0 { &LOOK_RINGS } else { &FIRST_RINGS };
        for &r in rings.iter() {
            if r > reach + 0.4 { break; }
            for k in 0..turns {
                let sign = if k % 2 != 0 { -1.0 } else { 1.0 };
                let a = a0 + (if look != 0 { m::PI / 32.0 } else { 0.0 }) + sign * m::ceil(k as f64 / 2.0) * step;
                let p: V3 = [cpos[0] + m::sin(a) * r, 0.0, cpos[2] + m::cos(a) * r];
                if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { continue; }
                if let Some(w) = wise {
                    if !holds(p, true) { continue; }
                    if let Some(n) = w.not { if m::hyp2(p[0] - n[0], p[2] - n[2]) < 0.8 { continue; } }
                }
                let mut barred = false;
                if let Some(ap) = wise.and_then(|w| w.apart.as_ref()) {
                    for &q in &ap.stay { if shoulders(g, cpos, p, q) { barred = true; break; } }
                }
                if crate::scene_collision::solid_blocked(g, [cpos[0], 0.45, cpos[2]], [p[0], 0.45, p[2]], None) { continue; }
                let mut clear = true;
                let mut s = 0.4;
                while s < r {
                    if crate::scene_collision::nav_grid_is_blocked(g, cpos[0] + m::sin(a) * s, cpos[2] + m::cos(a) * s) { clear = false; break; }
                    s += 0.4;
                }
                if !clear { continue; }
                if let Some(l) = &leaves { if leg_by_leaf(g, l, cpos, p, leaf0) { continue; } }
                let hidden = exposed_to(g, from, p, [p[0], 1.15, p[2]], None) <= 0.0;
                let dp = m::hyp2(p[0] - at[0], p[2] - at[2]);
                let shaded = !hidden && flee_shade(g, from, p);
                if wise.is_some() && !hidden && !shaded && dp <= d0 + 0.3 { continue; }
                if look != 0 && !hidden && !shaded && dp <= wound_m { continue; }
                if !barred && hidden {
                    if let Some(ap) = wise.and_then(|w| w.apart.as_ref()) {
                        for &q in &ap.go {
                            if m::hyp2(p[0] - q[0], p[2] - q[2]) < flee_apart_m && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None) { barred = true; break; }
                        }
                    }
                }
                let s0 = score(hidden, shaded, r, dp);
                let sc = if hidden && s0 > best_s {
                    // seen: a spot past his legs pays the whole cost; no asker, nothing
                    let cost = match ask.as_mut() { Some(f) => flee_cost * (if r > reach { 1.0 } else { crate::picture::exposure_ask(g, f, p) }), None => 0.0 };
                    s0 - cost
                } else { s0 };
                if barred {
                    if look == 0 {
                        mates = mates || hidden;
                        if s0 > bar_s { bar_s = s0; bar = Some(p); }
                    }
                    continue;
                }
                next_id += 1;
                if mind && s0 > plain_s { plain_s = s0; plain_id = next_id; }
                if sc > best_s { best_s = sc; best = Some(p); best_id = next_id; best_open = !hidden && !shaded; }
            }
        }
    }
    if (best.is_none() || best_open) && flee_shade(g, from, cpos) { return (None, false); }
    if best.is_none() && bar.is_some() && d0 < lethal_m { return (bar, false); }
    if mind && best.is_some() && best_id != plain_id && best_id != 1 { g.stats.picture.moved += 1; }
    (best, best.is_some() && best_id == 1)
}

/// A doorway a man can leave by as it stands.
fn open_way(g: &crate::game::Game, d: DoorId) -> bool {
    g.dr[d].leaf.off || (!g.dr[d].leaf.latched && !g.dr[d].leaf.closing && m::abs(g.dr[d].leaf.angle) >= g.k.frag.flee_door_open)
}

/// His spot is in the mouth of an open doorway.
pub(super) fn in_mouth(g: &crate::game::Game, p: V3) -> bool {
    for d in g.doors.list.clone() {
        if open_way(g, d) && flat(g.dr[d].frame_centre, p) < g.k.frag.flee_door_mouth { return true; }
    }
    false
}

/// what fleeByDoor works out once, for its local closures (near, clear, walk, take)
struct DoorDeal<'a> {
    at: V3,
    d0: f64,
    path0: f64,
    wise: &'a FleeByDoorWise,
    leaves: Vec<Vec<f64>>,
    who: Option<GuardId>,
    ours: bool,
    best: Option<V3>,
    best_s: f64,
    via: bool,
}
fn door_off_path(g: &Game, x: f64, z: f64, path: &[V3]) -> f64 {
    let mut d = g.k.frag.lethal_m;
    for q in path { d = m::min(d, m::hyp2(x - q[0], z - q[2])); }
    d
}
fn door_near(g: &Game, k: &DoorDeal, x: f64, z: f64) -> bool {
    m::hyp2(x - k.at[0], z - k.at[2]) < k.d0 - g.k.frag.flee_door_dip || door_off_path(g, x, z, &k.wise.path) < k.path0 - 0.3
}
/// One straight leg: 0 clear, 1 a step of it is nearer the thing, 2 a leaf stands in it.
fn door_clear(g: &Game, k: &DoorDeal, a: V3, b: V3, ref_: f64) -> i32 {
    let r = flat(a, b);
    let mut t = 0.25;
    while t < r + 0.25 {
        let u = if r > 1e-3 { m::min(1.0, t / r) } else { 1.0 };
        if door_near(g, k, a[0] + (b[0] - a[0]) * u, a[2] + (b[2] - a[2]) * u) { return 1; }
        t += 0.25;
    }
    if leg_by_leaf(g, &k.leaves, a, b, ref_) { 2 } else { 0 }
}
/// A point of the path lies on the door's own frame line, in the opening.
fn door_on_frame(g: &Game, b: V3, x: DoorId) -> bool {
    let f = crate::guards::frame_of(g, x);
    if m::abs((b[0] - f.hx) * f.nx + (b[2] - f.hz) * f.nz) > 1e-4 { return false; }
    let al = (b[0] - f.hx) * f.ux + (b[2] - f.hz) * f.uz;
    al >= -0.1 && al <= f.w + 0.1
}
/// The floor's own way from `a` to `p` through doorway `d`: its length, or -1 (no way, a door not open on it, a step nearer the thing, not through `d`), -2 (a leaf stands in it).
fn door_walk(g: &mut Game, k: &DoorDeal, a: V3, p: V3, d: DoorId) -> f64 {
    let nodes = g.k.frag.flee_door_nodes;
    let Some(mut way) = find_way(g, a, p, nodes, k.who) else { return -1.0 };
    let (mut q, mut len, mut thru) = (a, 0.0, false);
    let ref_ = leaf_dist(&k.leaves, a[0], a[2]);
    way.push(p);
    let doors = g.doors.list.clone();
    for &b in &way {
        for &x in &doors {
            if crate::guards::crosses_door(g, q, b, x).is_some() || door_on_frame(g, b, x) {
                if x == d { thru = true; } else if !open_way(g, x) { return -1.0; }
            }
        }
        let cl = door_clear(g, k, q, b, ref_);
        if cl != 0 { return -(cl as f64); }
        len += flat(q, b);
        q = b;
    }
    if thru { len } else { -1.0 }
}
fn door_take(g: &mut Game, k: &mut DoorDeal, goal: V3, p: V3, len: f64, v: bool) {
    let keep = k.wise.keep;
    let same = match keep { Some(q) => flat(goal, q) < 0.05, None => false };
    let unknown = if !k.ours || crate::cleared::cleared_rooms_known_at(g, p) { 0.0 } else { g.k.frag.flee_door_new };
    let sc = 100.0 - len * 2.0 + m::min(flat(p, k.at), 4.0) * 0.2 - unknown + (if same { g.k.frag.flee_keep } else { 0.0 });
    if sc > k.best_s {
        k.best_s = sc;
        k.best = if same { keep } else { Some(goal) };
        k.via = v;
    }
}

/// The spot beside an open doorway, on its far side, that the grenade has no line to and his legs make by the floor's own path.
pub(super) fn flee_by_door(g: &mut crate::game::Game, c_2: crate::frag::FleeByDoorC, at: V3, speed: f64, left: f64, accel: f64, wise: &crate::frag::FleeByDoorWise, who: Option<GuardId>) -> Option<crate::frag::FleeByDoor> {
    let cpos = c_2.pos;
    let from: V3 = [at[0], at[1] + g.k.frag.lift, at[2]];
    let tt = m::max(0.0, left - 0.15);
    let ta = speed / accel;
    let reach = (if tt <= ta { 0.5 * accel * tt * tt } else { speed * (tt - ta / 2.0) }) * g.k.frag.flee_door_in;
    if !(reach > 0.5) { return None; }
    let d0 = flat(cpos, at);
    let path0 = door_off_path(g, cpos[0], cpos[2], &wise.path);
    let leaves = open_leaves(g, cpos, reach + 2.0);
    let ours = match who { None => true, Some(w) => g.team(w).ours() };
    let mut k = DoorDeal { at, d0, path0, wise, leaves, who, ours, best: None, best_s: f64::NEG_INFINITY, via: false };
    let (most, tries, apart_m) = (g.k.frag.flee_door_most, g.k.frag.flee_door_try, g.k.frag.flee_apart_m);
    let mut open: Vec<DoorId> = Vec::new();
    for d in g.doors.list.clone() {
        if open_way(g, d) && flat(g.dr[d].frame_centre, cpos) <= reach { open.push(d); }
    }
    js::sort_by(&mut open, |a, b| flat(g.dr[*a].frame_centre, cpos) - flat(g.dr[*b].frame_centre, cpos));
    let mut asked = 0.0;
    for d in open {
        let fc = g.dr[d].frame_centre;
        let w = g.dr[d].def.width;
        let th = g.dr[d].def.closed_dir;
        let (ux, uz) = (m::cos(th), m::sin(th));
        let (nx, nz) = (-uz, ux);
        let side = if (cpos[0] - fc[0]) * nx + (cpos[2] - fc[2]) * nz >= 0.0 { 1.0 } else { -1.0 };
        if ((at[0] - fc[0]) * nx + (at[2] - fc[2]) * nz) * side < 0.0 { continue; }
        let was = asked;
        asked += 1.0;
        if was >= most { break; }
        let mut spots: Vec<V3> = Vec::new();
        for kk in [1.0, -1.0] { for a in [0.4, 0.8, 1.6, 2.4] { for o in [0.7, 1.1] {
            let s = w / 2.0 + a;
            let p: V3 = [fc[0] + ux * s * kk - nx * o * side, 0.0, fc[2] + uz * s * kk - nz * o * side];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || flat(p, at) < d0 - 0.05 { continue; }
            if let Some(n) = wise.not { if flat(p, n) < 0.8 { continue; } }
            if let Some(ap) = &wise.apart {
                if ap.go.iter().any(|&q| flat(p, q) < apart_m) || ap.stay.iter().any(|&q| flat(p, q) < apart_m) { continue; }
            }
            if exposed_to(g, from, p, [p[0], 1.15, p[2]], None) == 0.0 { spots.push(p); }
        } } }
        js::sort_by(&mut spots, |a, b| flat(*a, cpos) - flat(*b, cpos));
        let (mut got, mut leafed, mut ways) = (false, false, 0.0);
        for &p in &spots {
            if ways >= tries || flat(p, cpos) > reach { break; }
            let len = door_walk(g, &k, cpos, p, d);
            if len == -2.0 { leafed = true; }
            if len < 0.0 || len > reach { continue; }
            got = true;
            ways += 1.0;
            door_take(g, &mut k, p, p, len, false);
        }
        if got || !leafed || g.dr[d].leaf.off { continue; }
        // behind its own open leaf: round the leaf's end first, to the floor in front of the doorway, then out
        let u = crate::doors::door_dir(g, d);
        let h = g.dr[d].def.hinge;
        let tip_n = m::max(0.0, ((h[0] + u[0] * w - fc[0]) * nx + (h[1] + u[1] * w - fc[2]) * nz) * side);
        for mm in [0.65, 1.0] {
            let sp: V3 = [fc[0] + nx * side * (tip_n + mm), 0.0, fc[2] + nz * side * (tip_n + mm)];
            let l0 = flat(cpos, sp);
            if l0 < g.k.frag.flee_door_via_m || crate::scene_collision::nav_grid_is_blocked(g, sp[0], sp[2]) || !crate::scene_collision::nav_grid_walkable(g, cpos, sp) { continue; }
            let ref_ = leaf_dist(&k.leaves, cpos[0], cpos[2]);
            if door_clear(g, &k, cpos, sp, ref_) != 0 { continue; }
            if let Some(n) = wise.not { if flat(sp, n) < 0.3 { continue; } }
            let mut any = false;
            let mut n = 0.0;
            for &p in &spots {
                if n >= tries { break; }
                let len = door_walk(g, &k, sp, p, d);
                if len < 0.0 || l0 + len > reach * g.k.frag.flee_door_via { continue; }
                any = true;
                n += 1.0;
                door_take(g, &mut k, sp, p, l0 + len, true);
            }
            if any { break; }
        }
    }
    k.best.map(|goal| FleeByDoor { goal, via: k.via })
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own bits, under EXACT
    #[test]
    fn leaf_dist_bits() {
        let l = vec![vec![1.25, -3.5, 2.1, -2.7], vec![0.3, 0.7, 0.3, 1.9]];
        assert_eq!(leaf_dist(&l, 1.9, -1.3).to_bits(), 0x3ff6a09e667f3bcf);
        assert_eq!(leaf_dist(&l, -0.4, 1.1).to_bits(), 0x3fe6666666666666);
        assert_eq!(leaf_dist(&[], 1.0, 1.0).to_bits(), 0x7ff0000000000000);
    }
}
