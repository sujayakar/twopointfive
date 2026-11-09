#![allow(unused_imports, unused_variables)]
pub use ::game_types::runner::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// the script runner.ts lent for the frame (guards.ts:2144), as the run left it, put back on his row (conventions 4; see morale's b_01).
pub(super) fn runner_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(r)) = (ran, g.m.runner.state_of.get_mut(gd)) { r.script = ran; }
}

/// the man's own row of `stateOf`: every `S` of the TypeScript is that row, reached by its key
macro_rules! run {
    ($g:expr, $gd:expr) => {
        $g.m.runner.state_of.get_mut($gd).unwrap()
    };
}

// ---- the lines

/// one kind's row of the skeleton's table   (runner.ts LINES)
fn lines_of(kind: LineKind) -> LINESValue {
    lines().get(String::from(kind.as_str())).unwrap().clone()
}

/// one line of a kind from this man, shouted, never on the net   (runner.ts bark)
fn bark(g: &mut crate::game::Game, gd: GuardId, kind: crate::runner::LineKind) {
    let l = lines_of(kind);
    let extra = if g.gd[gd].team != crate::teams::Team::Guard { crate::words::w(g, &l.they, &l.he) } else { l.he };
    let mut pool = l.say;
    pool.extend(extra.unwrap_or_default());
    let n = pool.len() as f64;
    let at = js::rem(m::floor(crate::dice::draw_of(g, "runner::bark#1", gd) * n), n) as usize;
    crate::game::game_say(g, gd, &pool[at], Some(false), Some(true));
}

/// distance on the floor   (runner.ts flat)
fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// the chest bone, or where it would be   (runner.ts chestOf)
fn chest_of(g: &crate::game::Game, c_2: CharId) -> V3 {
    if let Some(b) = g.ch[c_2].bones.get(crate::character::Bone::Chest) { return *b; }
    let p = g.ch[c_2].pos;
    [p[0], p[1] + 1.2, p[2]]
}

/// the head bone, or where it would be   (runner.ts headOf)
fn head_of(g: &crate::game::Game, c_2: CharId) -> V3 {
    if let Some(b) = g.ch[c_2].bones.get(crate::character::Bone::Head) { return *b; }
    let p = g.ch[c_2].pos;
    [p[0], p[1] + 1.6, p[2]]
}

/// a living defender or a man of the night staff: never a civilian, an operative, a body   (runner.ts ours)
fn ours(g: &crate::game::Game, gd: GuardId) -> bool {
    let o = &g.gd[gd];
    o.state != crate::game::GuardState::Dead && o.civ.is_none() && (o.team == crate::teams::Team::Hostile || o.team == crate::teams::Team::Guard)
}

/// nobody's to take: the hostage rescue's cast and the response team   (runner.ts spoken)
fn spoken(g: &crate::game::Game, gd: GuardId) -> bool {
    (g.rescue.active && g.rescue.hostiles.contains(&gd)) || crate::qrf::qrf_owns(g, gd)
}

/// may the sequence start on him this frame   (runner.ts reacts)
fn reacts(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.ai_enabled || g.gd[gd].held.is_some() { return false; }
    g.gd[gd].script.is_none() || g.fight.defs.iter().any(|d| g.tab.roomfight_def[*d].gd == gd)
}

/// a living armed man of his own side, on his feet: somebody to run to, somebody to tell   (runner.ts mateOf)
fn mate_of(g: &crate::game::Game, gd: GuardId, o: GuardId) -> bool {
    o != gd && ours(g, o) && g.gd[o].team == g.gd[gd].team && g.gd[o].held.is_none() && !g.m.runner.state_of.has(o)
}

/// an enemy as a run reckons with him   (runner.ts foeAt)
fn foe_at(g: &crate::game::Game, c_2: CharId, who: crate::picture::Who) -> crate::runner::Foe {
    let eye = head_of(g, c_2);
    Foe { pos: g.ch[c_2].pos, eye, who }
}

/// his enemies on the floor: the player and every living man of a side hostile to his   (runner.ts foes)
pub fn foes(g: &crate::game::Game, gd: GuardId) -> Vec<crate::runner::Foe> {
    let mut out = Vec::new();
    let team = g.gd[gd].team;
    if crate::teams::is_hostile(g, team, crate::teams::Team::Player) {
        // (a man's picture holds one place for "the player": the one nearest him)
        if let Some(pl) = crate::seats::nearest_man(g, &g.ch[g.gd[gd].ch].pos) { let c = g.pl[pl].ch; out.push(foe_at(g, c, crate::picture::Who::Player)); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ot = g.gd[o].team;
        if g.gd[o].state != crate::game::GuardState::Dead && crate::teams::is_hostile(g, team, ot) {
            let c = g.gd[o].ch;
            out.push(foe_at(g, c, crate::picture::Who::Guard(o)));
        }
    }
    out
}

/// the enemies a run is planned round: the men he knows of, each where he believes him to be   (runner.ts known)
fn known(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::runner::Foe> {
    let mut out = Vec::new();
    let (forget, eye_h) = (g.k.picture.forget_secs, g.k.picture.eye_h);
    for i in crate::picture::ideas_of(g, gd) {
        if !i.lost && g.time - i.t < forget { out.push(Foe { pos: i.at, eye: [i.at[0], eye_h, i.at[2]], who: i.who }); }
    }
    out
}

/// the nearest enemy plainly in his eyes inside `closeM`; nobody for a man with magnesium in his eyes   (runner.ts closeFoe)
fn close_foe(g: &mut crate::game::Game, gd: GuardId) -> Option<CharId> {
    if crate::control::man::blind(g, gd) { return None; }
    let mut best: Option<CharId> = None;
    let mut bd = g.k.runner.close_m;
    let p = crate::seats::faced_man(g, gd);
    let team = g.gd[gd].team;
    if crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !g.pl[p].down && !g.player_invisible {
        let vis = crate::guards::can_see(g, gd);
        if vis.visible && vis.dist <= bd && crate::guards::sight_gain(g, gd, vis.dist) > 0.25 {
            best = Some(g.pl[p].ch);
            bd = vis.dist;
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let ot = g.gd[o].team;
        if g.gd[o].state == crate::game::GuardState::Dead || !crate::teams::is_hostile(g, team, ot) { continue; }
        let c = g.gd[o].ch;
        let vis = crate::guards::can_see_char(g, gd, c, false);
        if vis.visible && vis.dist <= bd && crate::guards::sight_gain_on(g, gd, o, vis.dist) > 0.25 {
            best = Some(g.gd[o].ch);
            bd = vis.dist;
        }
    }
    best
}

/// is he the man: broken, alone, dry, and nobody else's   (runner.ts ripe)
fn ripe(g: &crate::game::Game, gd: GuardId) -> bool {
    if !ours(g, gd) || spoken(g, gd) || !reacts(g, gd) || crate::morale::rung_of(g, gd) != 2.0 { return false; }
    let mo = match crate::morale::morale_of(g, gd) { Some(x) if x.alone => x, _ => return false };
    if mo.act == crate::morale::MoraleAct::Bait { return false; }
    if mo.act == crate::morale::MoraleAct::Reload { return mo.t >= g.k.runner.out_secs; }
    let w = g.gd[gd].weapon;
    crate::weapons::firearm_rounds_ready(g, w) == 0.0
}

/// or the man his leader's fall left: broken, green, and he saw it   (runner.ts lost)
fn lost(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !ours(g, gd) || spoken(g, gd) || !reacts(g, gd) || crate::morale::rung_of(g, gd) != 2.0 { return false; }
    crate::morale::morale_of(g, gd).map_or(false, |x| x.green) && crate::morale::saw_leader_fall(g, gd).is_finite()
}

/// the shout: from here his body is this file's, and morale.ts's sequence is over   (runner.ts start)
fn start(g: &mut crate::game::Game, gd: GuardId, from: V3) {
    let c = g.gd[gd].ch;
    let at = v3::copy(g.ch[c].pos);
    let (rule, now) = (crate::owner::by::RUNNER, g.time);
    crate::owner::claim(g, gd, rule, now, None);
    crate::lanes::lane_say(g, gd);   // (a man who has run holds no lane)
    g.m.runner.state_of.set(gd, Run {
        phase: RunPhase::Say, t: 0.0, from: [from[0], 0.0, from[2]], goal: None, mate: None, told: 0.0, unseen: 0.0, at, stall: 0.0, is_out: false,
        tries: 0.0, plan: 0.0, face: None, bold: false, bare: false, drive: false, drive_t: -1.0, script: crate::game::GuardScript::default(),
    });
    crate::morale::morale_release(g, gd);
    g.gd[gd].reload_t = -1.0;
    crate::character::anim_cancel_reload(g, c);
    let w = g.gd[gd].weapon;
    g.fa[w].burst = None;
    bark(g, gd, LineKind::Done);
    g.stats.runner.said += 1;
}

/// start the sequence on this man directly. False = he is not a man of this file's, or it already has him   (runner.ts runnerBreak)
pub fn runner_break(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !ours(g, gd) || spoken(g, gd) || !reacts(g, gd) || g.m.runner.state_of.has(gd) { return false; }
    let pos = g.ch[g.gd[gd].ch].pos;
    let mut from = g.gd[gd].last_known.unwrap_or(pos);
    let mut bd = m::INF;
    for e in known(g, gd) {
        let d = flat(e.pos, pos);
        if d < bd {
            bd = d;
            from = e.pos;
        }
    }
    start(g, gd, from);
    true
}

/// the rectangle round every room of the level's structure [x0, x1, z0, z1], worked out once a level   (runner.ts rectOf)
fn rect_of(g: &mut crate::game::Game) -> Option<[f64; 4]> {
    let none = match g.level.scenarios.as_ref().and_then(|s| s.room_clear.as_ref()) { Some(s) => s.rooms.is_empty(), None => true };
    if none { return None; }
    if let Some(r) = g.m.runner.rects { return Some(r); }
    let mut r = [m::INF, -m::INF, m::INF, -m::INF];
    for room in &g.level.scenarios.as_ref().unwrap().room_clear.as_ref().unwrap().rooms {
        r[0] = m::min(r[0], room.bounds.x0);
        r[1] = m::max(r[1], room.bounds.x1);
        r[2] = m::min(r[2], room.bounds.z0);
        r[3] = m::max(r[3], room.bounds.z1);
    }
    g.m.runner.rects = Some(r);
    Some(r)
}

/// out of the building: outside the rooms' rectangle by `clearM` under open sky; with no structure, the rim of the walkable map   (runner.ts outside)
fn outside(g: &mut crate::game::Game, p: V3) -> bool {
    let (clear, edge) = (g.k.runner.clear_m, g.k.runner.edge_m);
    let r = rect_of(g);
    let r = match r {
        Some(r) => r,
        None => {
            let nav = &g.col.nav;
            return m::min_n(&[p[0], p[2], nav.w * nav.cell - p[0], nav.h * nav.cell - p[2]]) <= edge;
        }
    };
    if p[0] >= r[0] - clear && p[0] <= r[1] + clear && p[2] >= r[2] - clear && p[2] <= r[3] + clear { return false; }
    let mut roofed = 0;
    for (dx, dz) in [(0.0, 0.0), (0.35, 0.0), (-0.35, 0.0), (0.0, 0.35), (0.0, -0.35)] {
        if crate::scene_collision::solid_blocked(g, [p[0] + dx, 1.9, p[2] + dz], [p[0] + dx, 9.0, p[2] + dz], Some(0.02)) { roofed += 1; }
    }
    roofed < 3
}

/// has this enemy a line to a man at `p`   (runner.ts lineTo)
fn line_to(g: &mut crate::game::Game, e: crate::runner::Foe, p: V3) -> bool {
    !crate::scene_collision::segment_blocked(g, e.eye, [p[0], p[1] + 1.2, p[2]], None, None, None)
}

/// the nearest enemy to a point   (runner.ts flood's near)
fn near(e: &[Foe], p: V3) -> f64 {
    let mut d = m::INF;
    for x in e { d = m::min(d, flat(x.pos, p)); }
    d
}

/// the floor he can reach, flooded once from where he stands: the nearest outside, for choice unseen, and the farthest corner   (runner.ts flood)
fn flood(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe], ask: Option<f64>, see: Option<&[crate::runner::Foe]>) -> crate::runner::Flood {
    let ask = ask.unwrap_or(g.k.runner.ask_cells);
    let see = see.unwrap_or(e);
    let (shy, corner_m) = (g.k.runner.shy_m, g.k.runner.corner_m);
    let (cs, wf, hf) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (wi, hi) = (wf as i64, hf as i64);
    let pos = g.ch[g.gd[gd].ch].pos;
    let from = crate::scene_collision::nav_grid_nearest_free_point(g, pos[0], pos[2]).unwrap_or(pos);
    let cells = (wi * hi) as usize;
    let mut depth = vec![-1i32; cells];
    let mut queue = vec![0usize; cells];
    let (mut head, mut tail) = (0usize, 0usize);
    let s = crate::scene_collision::nav_grid_idx(g, m::min(wf - 1.0, m::max(0.0, m::floor(from[0] / cs))), m::min(hf - 1.0, m::max(0.0, m::floor(from[2] / cs)))) as usize;
    depth[s] = 0;
    queue[tail] = s;
    tail += 1;
    let mut out: Option<V3> = None;
    let mut seen_out: Option<V3> = None;
    let mut asked = 0.0;
    let mut corner = v3::copy(pos);
    let mut cd = -m::INF;
    let mut shied = false;
    let far = m::round(corner_m / cs);
    while head < tail && out.is_none() {
        let i = queue[head];
        head += 1;
        let x = i as i64 % wi;
        let z = (i as i64 - x) / wi;
        let p: V3 = [(x as f64 + 0.5) * cs, 0.0, (z as f64 + 0.5) * cs];
        if outside(g, p) {
            if seen_out.is_none() { seen_out = Some(p); }
            if x & 1 == 0 && z & 1 == 0 {   // (every other cell each way: a metre's grid of them is asked for a line)
                let was = asked;
                asked += 1.0;
                if was >= ask { break; }
                if !see.iter().any(|f| line_to(g, *f, p)) {
                    out = Some(p);
                    break;
                }
            }
        }
        let mut walls = 0;
        let dp = near(e, p);
        for (dx, dz) in [(1i64, 0i64), (-1, 0), (0, 1), (0, -1)] {
            let (nx, nz) = (x + dx, z + dz);
            if nx < 0 || nz < 0 || nx >= wi || nz >= hi {
                walls += 1;
                continue;
            }
            let j = crate::scene_collision::nav_grid_idx(g, nx as f64, nz as f64) as usize;
            if g.col.nav.blocked[j] != 0 {
                walls += 1;
                continue;
            }
            if depth[j] >= 0 { continue; }
            let dn = near(e, [(nx as f64 + 0.5) * cs, 0.0, (nz as f64 + 0.5) * cs]);
            if dn < shy && dn <= dp {
                shied = true;
                continue;
            }
            depth[j] = depth[i] + 1;
            queue[tail] = j;
            tail += 1;
        }
        if walls >= 2 && depth[i] as f64 <= far {
            let d = near(e, p);
            if d > cd {
                cd = d;
                corner = p;
            }
        }
    }
    Flood { out: out.or(seen_out), hidden: out.is_some(), corner, shied }
}

/// his way to `goal`, with every cell inside `shyM` of an enemy made dear, doors threaded   (runner.ts route)
fn route(g: &mut crate::game::Game, gd: GuardId, goal: V3, e: &[crate::runner::Foe]) -> bool {
    let shy = g.k.runner.shy_m;
    let cs = g.col.nav.cell;
    let pos = g.ch[g.gd[gd].ch].pos;
    g.stats.picture.ran += 1;
    let t = foes(g, gd);
    if t.len() != e.len() || t.iter().any(|t| !e.iter().any(|x| flat(x.pos, t.pos) < 1.0)) { g.stats.picture.ran_off += 1; }
    let mut pen = |_g: &mut Game, ix: f64, iz: f64| -> f64 {
        let (x, z) = ((ix + 0.5) * cs, (iz + 0.5) * cs);
        for f in e {
            if m::hyp2(f.pos[0] - x, f.pos[2] - z) < shy { return 20.0; }
        }
        1.0
    };
    let pen_arg: Option<&mut dyn FnMut(&mut Game, f64, f64) -> f64> = if e.is_empty() { None } else { Some(&mut pen) };
    let p = crate::scene_collision::nav_grid_find_path(g, pos, goal, Some(12000.0), pen_arg);
    {
        let r = run!(g, gd);
        r.goal = Some(v3::copy(goal));
        r.script.arrived = Some(false);
        r.at = v3::copy(pos);
        r.stall = 0.0;
    }
    // (`S.goal = v3.copy(goal)` is a NEW ARRAY, and his script's goal is that object from the next drive on: the settle watch squad.ts:81 keeps for
    //  the goal OBJECT it was made for (`w.goal !== s.goal`) is made again at its next read, even where the route is to the same spot. Here the watch is
    //  asked by value, so only a watch that holds THESE numbers would be taken for this goal's: that one goes. Any other stays as the TypeScript's
    //  stale row stays, and the next read replaces it. STAND-IN by value: with the same numbers the TypeScript keeps a stale row until that read and
    //  this side has none; it belongs to the identity device.)
    if g.m.squad.settle_watch.get(gd).map_or(false, |w| w.goal == goal) { g.m.squad.settle_watch.delete(gd); }
    let mut free = |g: &mut Game, x: f64, z: f64| -> bool { !crate::scene_collision::nav_grid_is_blocked(g, x, z) };
    let path = crate::doors::doors_thread_path(g, p.as_deref().unwrap_or(&[]), pos, Some(&mut free));
    let o = &mut g.gd[gd];
    o.path = path;
    o.path_serial = o.path_serial.wrapping_add(1);   // (a new array: readers ask by identity. The man's own count, as every other whole write of his path: a number of the game's counter here could come back to one a reader kept)
    o.path_i = 0;
    o.path_goal = Some(v3::copy(goal)); o.path_goal_orbit = false;   // (guard_set_path_goal's rule, for the row in hand)
    o.repath_t = 0.0;
    o.line_side = 0.0;
    p.map_or(false, |p| !p.is_empty())
}

/// a path's length on the floor   (runner.ts pathLen)
fn path_len(from: V3, path: &[V3]) -> f64 {
    let mut l = 0.0;
    let mut a = from;
    for b in path {
        l += flat(a, *b);
        a = *b;
    }
    l
}

/// the mate he runs to: the nearest by the floor, inside `mateM`, of the four nearest by air   (runner.ts pickMate)
fn pick_mate(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let pos = g.ch[g.gd[gd].ch].pos;
    let mate_m = g.k.runner.mate_m;
    let mut best: Option<GuardId> = None;
    let mut bl = mate_m;
    let mut men: Vec<(GuardId, f64)> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if !mate_of(g, gd, o) { continue; }
        let d = flat(g.ch[g.gd[o].ch].pos, pos);
        if d <= mate_m { men.push((o, d)); }
    }
    js::sort_by(&mut men, |a, b| a.1 - b.1);
    men.truncate(4);
    for (o, d) in men {
        if d >= bl { break; }
        let op = g.ch[g.gd[o].ch].pos;
        let p = match crate::scene_collision::nav_grid_find_path(g, pos, op, None, None) { Some(p) if !p.is_empty() => p, _ => continue };
        let l = path_len(pos, &p);
        if l <= bl && (best.is_none() || l < bl) {
            best = Some(o);
            bl = l;
        }
    }
    best
}

/// is there an enemy in the `aheadM` of his way in front of him, nearer to it than to him   (runner.ts wayBarred)
fn way_barred(g: &crate::game::Game, gd: GuardId, e: &[crate::runner::Foe]) -> bool {
    if e.is_empty() { return false; }
    let (shy, ahead) = (g.k.runner.shy_m, g.k.runner.ahead_m);
    let pos = g.ch[g.gd[gd].ch].pos;
    let o = &g.gd[gd];
    let mut a = pos;
    let mut left = ahead;
    let mut i = o.path_i;
    while i < o.path.len() && left > 0.0 {
        let b = o.path[i];
        i += 1;
        let l = flat(a, b);
        if l < 1e-4 { continue; }
        let use_ = m::min(l, left);
        let (ux, uz) = ((b[0] - a[0]) / l, (b[2] - a[2]) / l);
        for f in e {
            let t = m::min(use_, m::max(0.0, (f.pos[0] - a[0]) * ux + (f.pos[2] - a[2]) * uz));
            let d = m::hyp2(f.pos[0] - a[0] - ux * t, f.pos[2] - a[2] - uz * t);
            if d < shy && d < flat(f.pos, pos) - 0.25 { return true; }
        }
        left -= l;
        a = b;
    }
    false
}

/// the drop: the weapon on the floor, a calm man on the books, and off he goes   (runner.ts drop)
fn drop(g: &mut crate::game::Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    let def = g.fa[g.gd[gd].weapon].def;
    let (kind, suppressed) = {
        let w = g.m.weapons.weapons.get(def).unwrap();
        let kind = match w.drop {
            crate::slotsetup::SetupGun::Pistol => crate::combat::DropFromHandKind::Pistol,
            crate::slotsetup::SetupGun::Rifle => crate::combat::DropFromHandKind::Rifle,
        };
        (kind, w.suppressed)
    };
    let dir = g.ch[c].gun_dir;
    let item = crate::combat::drop_from_hand(g, c, kind, dir, [0.0, 0.0, 0.0], Some(suppressed), None, None);
    g.gd[gd].dropped = Some(item);
    if kind == crate::combat::DropFromHandKind::Rifle {   // (…and the sidearm off his thigh with it: he keeps nothing)
        let dir = g.ch[c].gun_dir;
        let from = match g.ch[c].bones.get(crate::character::Bone::ThighR) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 0.85, 0.0]) };
        crate::combat::drop_from_hand(g, c, crate::combat::DropFromHandKind::Pistol, dir, [0.0, 0.0, 0.0], Some(false), Some(true), Some(from));
        run!(g, gd).bare = true;
    }
    g.gd[gd].fled = true;
    let (rule, now) = (crate::owner::by::RUNNER, g.time);
    crate::owner::claim(g, gd, rule, now, Some(1.0));
    g.stats.runner.dropped += 1;
    {
        crate::mind::set_state(g, gd, crate::game::GuardState::Patrol, "runner");
        crate::mind::set(g, gd, 0.0, "runner");
        crate::mind::foe(g, gd, None, "runner");
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        g.gd[gd].pieing = None;
        g.gd[gd].search_plan = None;
        g.gd[gd].standoff = None;
        g.gd[gd].muzzle_down = false;
    }
    crate::control::set_hold(g, gd, true, "runner"); crate::control::set_pinned(g, gd, true, "runner");
    let e = known(g, gd);
    let mate = pick_mate(g, gd);
    if let Some(ma) = mate {
        let r = run!(g, gd);
        r.phase = RunPhase::Mates;
        r.t = 0.0;
        r.mate = Some(ma);
        let at = g.ch[g.gd[ma].ch].pos;
        route(g, gd, at, &e);
        g.stats.runner.to_mates += 1;
    } else {
        to_out(g, gd, &e);
    }
}

/// a flood's answer as plan_out reads it   (runner.ts:288 F)
fn plan_f(f: &Flood) -> PlanOutF { PlanOutF { out: f.out, shied: f.shied } }

/// out of the building by the nearest floor that is outside; no such floor: the corner   (runner.ts toOut)
fn to_out(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe]) {
    let f = flood(g, gd, e, None, None);
    {
        let r = run!(g, gd);
        r.mate = None;
        r.t = 0.0;
    }
    if plan_out(g, gd, e, plan_f(&f)) {
        run!(g, gd).phase = RunPhase::Out;
        g.stats.runner.out += 1;
    } else {
        to_low(g, gd, f.corner, e);
    }
}

/// to a corner, low   (runner.ts toLow)
fn to_low(g: &mut crate::game::Game, gd: GuardId, at: V3, e: &[crate::runner::Foe]) {
    {
        let r = run!(g, gd);
        r.phase = RunPhase::Low;
        r.t = 0.0;
        r.mate = None;
        r.bold = false;
    }
    route(g, gd, at, e);
    g.stats.runner.low += 1;
}

/// a way out, planned: round the enemies he knows of, else the nearest the floor itself gives him. True = he is on it   (runner.ts planOut)
fn plan_out(g: &mut crate::game::Game, gd: GuardId, e: &[crate::runner::Foe], f: crate::runner::PlanOutF) -> bool {
    if let Some(out) = f.out {
        if route(g, gd, out, e) {
            run!(g, gd).bold = false;
            return true;
        }
    }
    if !f.shied { return false; }   // (nobody's nearness cost the first look a cell: a second would find the same floor)
    let ask = g.k.runner.ask_cells;
    let b = match flood(g, gd, &[], Some(ask), Some(e)).out { Some(b) => b, None => return false };
    let was = *run!(g, gd);
    let watch_was = g.m.squad.settle_watch.get(gd).copied();   // (the goal object he had comes back below, and the watch made for it with it)
    let (path, path_serial, path_i, path_goal, repath_t, line_side) = {
        let o = &g.gd[gd];
        (o.path.clone(), o.path_serial, o.path_i, o.path_goal, o.repath_t, o.line_side)
    };
    if route(g, gd, b, e) {
        run!(g, gd).bold = true;
        g.stats.runner.bold += 1;
        return true;
    }
    {
        let r = run!(g, gd);
        r.goal = was.goal;
        r.script.arrived = was.script.arrived;
        r.at = was.at;
        r.stall = was.stall;
    }
    if let Some(w) = watch_was { g.m.squad.settle_watch.set(gd, w); }
    let o = &mut g.gd[gd];
    o.path = path;
    o.path_serial = path_serial;   // (the same array back: its old serial, none dealt)
    o.path_i = path_i;
    o.path_goal = path_goal; o.path_goal_orbit = false;   // (guard_set_path_goal's rule, for the row in hand)
    o.repath_t = repath_t;
    o.line_side = line_side;
    false
}

/// the mates are told where the team is: every mate inside `toldM` of him   (runner.ts tell)
fn tell(g: &mut crate::game::Game, gd: GuardId) {
    let (told_m, tell_beat, tell_aware) = (g.k.runner.told_m, g.k.runner.tell_beat, g.k.runner.tell_aware);
    let from = run!(g, gd).from;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if !mate_of(g, gd, o) || flat(g.ch[g.gd[o].ch].pos, g.ch[g.gd[gd].ch].pos) > told_m { continue; }
        let d = crate::roomfight::def_of(g, o);
        let on_post = match d { Some(_) => crate::roomfight::room_fight_told(g, o, from), None => false };
        if on_post {
            crate::digin::dig_in_order(g, d.unwrap(), from, tell_beat);
        } else {
            crate::belief::word(g, o, from);
            crate::mind::raise(g, o, tell_aware, "runner");
        }
        run!(g, gd).told += 1.0;
        g.stats.runner.told += 1;
    }
}

/// what this file's script holds him to this frame   (runner.ts pose)
fn pose(g: &mut crate::game::Game, gd: GuardId) {
    let pos = g.ch[g.gd[gd].ch].pos;
    let chase = g.tune.guard_chase;
    let fled = g.gd[gd].fled;
    let r = run!(g, gd);
    let say = r.phase == RunPhase::Say;
    let arrived = r.script.arrived.unwrap_or(false);
    let there = arrived && r.phase == RunPhase::Low;
    r.script.goal = if say { None } else { r.goal };
    r.script.speed = Some(chase);
    r.script.strafe = Some(false);
    r.script.face = if say { Some(v3::yaw_to(pos, r.from)) } else if there { r.face } else { None };
    r.script.aim_at = None;
    r.script.upper = Some(crate::character::UpperMode::None);
    r.script.stance = Some(if say { crate::character::Stance::LowReady } else { crate::character::Stance::None });
    r.script.crouch = Some(say || (arrived && (r.phase == RunPhase::Low || (r.phase == RunPhase::Out && r.unseen == 0.0))));
    r.script.over = None;
    if fled { r.script.light = Some(false); }
}

/// once a frame, after morale.ts's tick: the ripe men are asked for the enemy in their eyes, every man in a run takes a step   (runner.ts runnerTick)
pub fn runner_tick(g: &mut crate::game::Game, dt: f64) {
    let mut i = g.guards.len();
    while i > 0 {   // (backwards: a man gone leaves the list under it)
        i -= 1;
        let gd = g.guards[i];
        if let Some(r) = g.m.runner.state_of.get_mut(gd) {
            r.drive = false;
            if g.gd[gd].state != crate::game::GuardState::Dead { step(g, gd, dt); }
            continue;
        }
        if !(ripe(g, gd) || lost(g, gd)) { continue; }
        if let Some(c) = close_foe(g, gd) {
            let at = g.ch[c].pos;
            start(g, gd, at);
        }
    }
}

/// one frame of a man's run   (runner.ts step)
fn step(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let c = g.gd[gd].ch;
    crate::morale::morale_release(g, gd);
    if g.gd[gd].reload_t >= 0.0 {
        g.gd[gd].reload_t = -1.0;
        crate::character::anim_cancel_reload(g, c);
    }
    if run!(g, gd).phase == RunPhase::Say {
        if !reacts(g, gd) {
            g.m.runner.state_of.delete(gd);
            let (rule, now) = (crate::owner::by::RUNNER, g.time);
            crate::owner::release(g, gd, rule, now);
            return;
        }
        let beat = g.k.runner.beat_secs;
        let r = run!(g, gd);
        r.t += dt;
        if r.t < beat {
            drive(g, gd);
            return;
        }
        drop(g, gd);
    }
    let w = g.gd[gd].weapon;
    g.fa[w].chamber = 0.0;
    g.fa[w].mag = Some(0.0);
    g.fa[w].burst = None;
    crate::control::set_hold(g, gd, true, "runner");
    crate::control::set_pinned(g, gd, true, "runner");
    if g.gd[gd].state != crate::game::GuardState::Patrol { crate::mind::set_state(g, gd, crate::game::GuardState::Patrol, "runner"); }
    crate::mind::set(g, gd, 0.0, "runner");
    crate::mind::foe(g, gd, None, "runner");
    if g.gd[gd].held.is_some() { return; }
    let e = known(g, gd);
    run!(g, gd).t += dt;
    let (stall_secs, replan_secs, tell_m, mate_secs, gone_secs, retry_secs, ask_wide) = {
        let k = &g.k.runner;
        (k.stall_secs, k.replan_secs, k.tell_m, k.mate_secs, k.gone_secs, k.retry_secs, k.ask_wide)
    };
    let pos = g.ch[c].pos;
    let moving = {
        let r = run!(g, gd);
        !r.script.arrived.unwrap_or(false) && r.goal.is_some()
    };
    {
        let r = run!(g, gd);
        if moving && flat(pos, r.at) < 0.5 {
            r.stall += dt;
        } else {
            r.stall = 0.0;
            r.at = v3::copy(pos);
        }
    }
    let stuck = run!(g, gd).stall >= stall_secs;
    // the way ahead, looked at every `replanSecs`: an enemy has stepped into it since it was planned
    let mut barred = false;
    let r = run!(g, gd);
    r.plan += dt;
    if r.plan >= replan_secs {
        r.plan = 0.0;
        barred = moving && !r.bold && way_barred(g, gd, &e);
    }
    let row = *run!(g, gd);
    if row.phase == RunPhase::Mates {
        let gone_mate = match row.mate { None => true, Some(ma) => !mate_of(g, gd, ma) || !g.guards.contains(&ma) };
        if gone_mate || stuck || row.t > mate_secs {
            to_out(g, gd, &e);
        } else {
            let mc = g.gd[row.mate.unwrap()].ch;
            let mp = g.ch[mc].pos;
            if flat(pos, mp) <= tell_m && {
                let (a, b) = (head_of(g, c), chest_of(g, mc));
                !crate::scene_collision::segment_blocked(g, a, b, None, None, None)
            } {
                bark(g, gd, LineKind::Mates);
                tell(g, gd);
                to_out(g, gd, &e);
            } else if barred {
                to_out(g, gd, &e);   // (the enemy is between him and his own: he tells nobody)
            } else if flat(mp, row.goal.unwrap()) > 1.5 {
                route(g, gd, mp, &e);
            }
        }
    } else if row.phase == RunPhase::Out {
        let is_out = outside(g, pos);
        run!(g, gd).is_out = is_out;
        if is_out {
            // (the world's rule, not his: he leaves the floor when no TRUE enemy has a line to him)
            let eyes: Option<Vec<Foe>> = { let t = foes(g, gd);
                Some(t.into_iter().filter(|f| line_to(g, *f, pos)).collect()) };
            let seen = match &eyes {
                Some(v) => !v.is_empty(),
                None => e.iter().any(|f| line_to(g, *f, pos)),
            };
            {
                let r = run!(g, gd);
                r.unseen = if seen { 0.0 } else { r.unseen + dt };
            }
            let row = *run!(g, gd);
            if row.unseen >= gone_secs {
                gone(g, gd);
                return;
            }
            // out, there, and an enemy's eyes still on him: every `retrySecs` he looks wider for floor nobody has a line to
            if row.unseen == 0.0 && row.script.arrived.unwrap_or(false) && row.t >= retry_secs {
                run!(g, gd).t = 0.0;
                let mut e2 = e.clone();
                if let Some(v) = &eyes {
                    for f in v { crate::picture::picture_tell(g, gd, f.who, f.pos, None); }
                    e2 = known(g, gd);
                }
                let fl = flood(g, gd, &e2, Some(ask_wide), None);
                if let (true, Some(o)) = (fl.hidden, fl.out) {
                    if flat(o, pos) > 1.0 {
                        route(g, gd, o, &e2);
                        run!(g, gd).bold = false;
                    }
                }
            }
        } else if stuck || row.script.arrived.unwrap_or(false) {
            run!(g, gd).tries += 1.0;   // (a way that would not let him through: one try spent)
            let corner = flood(g, gd, &e, None, None).corner;
            to_low(g, gd, corner, &e);
        } else if barred {   // (another way out, or the corner, and no try spent)
            let fl = flood(g, gd, &e, None, None);
            if !plan_out(g, gd, &e, plan_f(&fl)) {
                to_low(g, gd, fl.corner, &e);
            }
        }
    } else if !row.is_out {   // low, in his corner or on his way to it
        if row.script.arrived.unwrap_or(false) && row.face.is_none() {
            let mut n: Option<Foe> = None;
            let mut nd = m::INF;
            for f in &e {
                let d = flat(f.pos, pos);
                if d < nd {
                    nd = d;
                    n = Some(*f);
                }
            }
            let yaw = match n { Some(n) => v3::yaw_to(n.pos, pos), None => g.ch[c].body_yaw };
            run!(g, gd).face = Some(yaw);
        }
        if barred {
            run!(g, gd).t = 0.0;
            let fl = flood(g, gd, &e, None, None);
            if flat(fl.corner, run!(g, gd).goal.unwrap_or(pos)) > 1.0 {
                route(g, gd, fl.corner, &e);
                run!(g, gd).face = None;
            }
        } else if stuck {
            route(g, gd, pos, &e);   // (a corner he cannot get to: he stays low where he has got to)
        } else if run!(g, gd).t >= retry_secs {   // the way out may have come free: he never settles, every look is another run at it
            run!(g, gd).t = 0.0;
            let fl = flood(g, gd, &e, None, None);
            if plan_out(g, gd, &e, plan_f(&fl)) {
                let r = run!(g, gd);
                r.phase = RunPhase::Out;
                r.face = None;
                g.stats.runner.out += 1;
            } else if flat(fl.corner, run!(g, gd).goal.unwrap_or(pos)) > 3.0 {   // (a failed route has written the goal: read it again)
                route(g, gd, fl.corner, &e);
                run!(g, gd).face = None;
            }
        }
    }
    drive(g, gd);
}

/// out and unseen: a spawned unit leaves the world; a man of the level's own roster stays low where he is   (runner.ts gone)
fn gone(g: &mut crate::game::Game, gd: GuardId) {
    g.stats.runner.gone += 1;
    let drilled = match crate::guards::engagement_of(g) { Some(x) => !x.done && x.hostiles.contains(&gd), None => false };
    if !drilled && crate::guards::despawn_unit(g, gd) {
        if let Some(r) = g.m.runner.state_of.get_mut(gd) {
            r.phase = RunPhase::Gone;
            r.drive = false;
        }
        return;
    }
    let pos = g.ch[g.gd[gd].ch].pos;
    let r = run!(g, gd);
    r.phase = RunPhase::Low;
    r.is_out = true;
    r.goal = Some(v3::copy(pos));
    r.script.arrived = Some(true);
    if g.m.squad.settle_watch.get(gd).map_or(false, |w| w.goal == pos) { g.m.squad.settle_watch.delete(gd); }   // (a new goal object, as `route` says)
    drive(g, gd);
}

/// this file's script has him this frame   (runner.ts drive)
fn drive(g: &mut crate::game::Game, gd: GuardId) {
    pose(g, gd);
    let now = g.time;
    let r = run!(g, gd);
    r.drive = true;
    r.drive_t = now;
}

/// WHAT IT WANTS OF HIM (a rule's second part, control/; it only reads): the same answer. Its state is moved by `runner_tick`.
pub(super) fn runner_propose(g: &crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> {
    let now = g.time;
    match g.m.runner.state_of.get(gd) {
        Some(r) if r.drive && r.drive_t == now => Some(r.script),
        _ => None,
    }
}

/// and his sidearm with it   (runner.ts runnerBare)
pub fn runner_bare(g: &crate::game::Game, gd: GuardId) -> bool { g.m.runner.state_of.get(gd).map_or(false, |r| r.bare) }

/// counts for nothing, from the drop   (runner.ts runnerNobody)
pub fn runner_nobody(g: &crate::game::Game, gd: GuardId) -> bool { g.gd[gd].fled }

/// speaks for nobody, from the shout   (runner.ts runnerMute)
pub fn runner_mute(g: &crate::game::Game, gd: GuardId) -> bool { let rule = crate::owner::by::RUNNER; crate::owner::has(g, gd, rule) }

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own bits, under bun with EXACT on
    #[test]
    fn flat_and_path_len_have_the_typescripts_bits() {
        assert_eq!(flat([1.1, 0.0, 2.2], [3.3, 7.0, -4.4]).to_bits(), 0x401bd3faa7236476);
        let path = [[1.7, 0.0, 3.9], [-2.3, 0.0, 4.1], [-2.3, 0.0, 4.1], [5.5, 1.0, -0.7]];
        assert_eq!(path_len([0.1, 0.0, 0.2], &path).to_bits(), 0x403131d9b6305e72);
    }

}
