//! src/game/combat.ts lines 663 to 756
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Seconds a stun can sits after it lands: FLASH.fuse in a room fight, else the night game's.   (combat.ts flashFuse)
pub fn flash_fuse(g: &mut crate::game::Game) -> f64 {
    if crate::roomfight::fight_live(g) { g.k.flash.fuse } else { crate::effects::bang().fuse }
}

/// The stun can goes off, as its thrower's doing.   (combat.ts detonateStun)
pub fn detonate_stun(g: &mut crate::game::Game, it: ItemId, borrowed: Option<bool>, sam: Option<crate::combat::DetonateStunSam>) {
    let borrowed = borrowed.unwrap_or(false);
    let raw = sam.map(|s| crate::combat::DetonateStunRawSam { side: s.side, spare: s.spare });
    let who = crate::throwables::item_who(g, it);
    crate::fogofwar::fog_of_war_doing(g, Some(who), &mut |g: &mut Game| detonate_stun_raw(g, it, borrowed, raw));
}

/// The burst, the flash and the bang, then who it blinds and folds: the guards, then every seat's man.   (combat.ts detonateStunRaw)
pub fn detonate_stun_raw(g: &mut crate::game::Game, it: ItemId, borrowed: bool, sam: Option<crate::combat::DetonateStunRawSam>) {
    let b = crate::effects::bang();
    let p = crate::throwables::item_pos(g, it);
    let at: V3 = [p[0], p[1] + b.height, p[2]];
    let tear = g.k.doorblast.tear_m.stun;
    let who = crate::throwables::item_who(g, it);
    crate::doors::doors_blast(g, at, tear, who);
    let gpu = crate::particle_fx::gpu_fx(g).is_some();
    let seed = g.fx_seed;
    g.fx_seed += 1.0;
    let o = if gpu {
        Some(crate::effects::SpawnBurstO { spark_count: Some(m::min(b.spark_count, if crate::effects::flags().trails { b.trail_count } else { 0.0 })) })
    } else {
        None
    };
    let l = crate::effects::spawn_burst(g, at, seed, o);
    if gpu { crate::particle_fx::bang_fx(g, at); }
    let flash = crate::render_lights::light_set_flash(&mut g.engine.lights, l.pos, l.color, l.intensity, l.range, l.ttl, 0.0);
    let light = crate::fogofwar::fog_of_war_light_by(g, flash, Some(who));
    if let Some(r) = l.radius {
        if js::truthy(r) { seam_light_set_radius(g, light, r); }
    }
    if !crate::blast_feel::wall_report(g, at) { crate::audio_iface::game_audio_like_play(g, "stunBang", Some(at), Some(1.0), None); }
    crate::marks::bang_scorch(g, at);
    crate::destruction::destruction_bang(g, at, Some(who), Some(1.0), Some(!borrowed));
    if borrowed { crate::destruction::destruction_furn_blast(g, at, crate::furnblast::BlastSource::Door, Some(who)); }
    {
        let ears = crate::seats::host_man(g);
        let c = g.pl[ears].ch;
        let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
            Some(h) => *h,
            None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]),
        };
        if !g.pl[ears].down && v3::dist(eye, at) < b.dazzle_radius && !seg(g, v3::add(at, [0.0, 0.3, 0.0]), eye) {
            crate::audio_iface::game_audio_like_play(g, "earRing", None, Some(v3::clamp(1.2 - v3::dist(eye, at) / b.dazzle_radius, 0.25, 0.8)), None);
        }
    }
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Bang,
        pos: at,
        time,
        loud: true,
        charge: if borrowed { Some(true) } else { None },
        serial,
        ..crate::game::GameEvent::zeroed()
    });
    let mut n: i32 = 0;
    let entry = crate::roomfight::fight_live(g);
    let side: crate::teams::Team = match g.tab.item[it].side {
        Some(s) => s,
        None => {
            let who = crate::throwables::item_who(g, it);
            if js::truthy(who) { crate::teams::team_of_id(g, who) } else { crate::teams::Team::Player }
        }
    };
    let from = v3::add(at, [0.0, 0.3, 0.0]);
    let over = v3::add(at, [0.0, g.k.flash.lift, 0.0]);
    let lifted = entry && !seg(g, from, over);
    if crate::effects::flags().dazzle {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
            let c = g.gd[gd].ch;
            let head = match g.ch[c].bones.get(crate::character::Bone::Head) {
                Some(h) => *h,
                None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
            };
            let d = v3::dist(head, at);
            if d > b.dazzle_radius { continue; }
            let secs: f64;
            if !entry {
                if seg(g, from, head) { continue; }
                secs = v3::lerp(b.dazzle_seconds, b.dazzle_min, v3::clamp(d / b.dazzle_radius, 0.0, 1.0));
            } else {
                let seen = !seg(g, from, head) || (lifted && !seg(g, over, head));
                let mut k = 1.0;
                if !seen {
                    let pr = crate::scene_collision::sound_paths_propagate(g, at, head, None);
                    if pr.muffle > 0.6 || pr.path_len > g.k.flash.concuss_m { continue; }
                    k = g.k.flash.concuss_k;
                }
                let u = v3::clamp(d / b.dazzle_radius, 0.0, 1.0);
                let team = g.gd[gd].team;
                if team == side
                    || (side == crate::teams::Team::Player && team == crate::teams::Team::Operative)
                    || (side == crate::teams::Team::Operative && team == crate::teams::Team::Player)
                {
                    if d > g.k.flash.own_m { continue; }
                    secs = g.k.flash.own_secs * k;
                } else {
                    secs = v3::lerp(g.k.flash.secs[0], g.k.flash.secs[1], u) * k;
                    let drilled = crate::roomfight::drilled_k(g, gd);
                    g.gd[gd].stagger_until = m::max(g.gd[gd].stagger_until, g.time + v3::lerp(g.k.flash.stagger[0], g.k.flash.stagger[1], u) * k * drilled);
                    let gun = g.gd[gd].weapon;
                    g.fa[gun].burst = None;
                }
            }
            g.gd[gd].dazzled_until = m::max(g.gd[gd].dazzled_until, g.time + secs);
            g.gd[gd].react_t = m::max(g.gd[gd].react_t, secs);
            g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.6);
            if g.gd[gd].state != crate::game::GuardState::Alert { g.gd[gd].last_known = Some(at); }
            if g.ai_enabled { g.gd[gd].witness.dazzled += 1.0; }
            g.gd[gd].path = Vec::new();
            g.gd[gd].path_goal = None;
            g.gd[gd].speed = 0.0;
            g.gd[gd].dazzled_at = Some(at);
            let along = v3::sub(g.ch[c].pos, at);
            crate::character::anim_hit_from(g, c, Some(along));
            n += 1;
        }
    }
    let sam_side: crate::teams::Team = sam.and_then(|s| s.side).unwrap_or(side);
    let spare = sam.and_then(|s| s.spare).unwrap_or(false);
    let mut told = false;
    on_each_seat(g, &mut |g: &mut Game| {
        let mut sam_secs = 0.0;
        let c = g.pl[g.player].ch;
        let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
            Some(h) => *h,
            None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]),
        };
        if !borrowed && crate::effects::flags().dazzle && !g.pl[g.player].down && g.puppet.is_none() && !g.touring {
            let d = v3::dist(eye, at);
            if d <= b.dazzle_radius {
                let u = v3::clamp(d / b.dazzle_radius, 0.0, 1.0);
                let mut stag = 0.0;
                if !entry {
                    if !seg(g, from, eye) { sam_secs = v3::lerp(b.dazzle_seconds, b.dazzle_min, u); }
                } else {
                    let seen = !seg(g, from, eye) || (lifted && !seg(g, over, eye));
                    let mut k = 1.0;
                    let mut reach = true;
                    let own = sam_side == crate::teams::Team::Player || sam_side == crate::teams::Team::Operative;
                    let full = !own || seen;
                    if !seen {
                        if own && spare {
                            reach = false;
                        } else {
                            let pr = crate::scene_collision::sound_paths_propagate(g, at, eye, None);
                            if pr.muffle > 0.6 || pr.path_len > g.k.flash.concuss_m { reach = false; }
                            k = g.k.flash.concuss_k;
                        }
                    }
                    if reach {
                        if !full {
                            if d <= g.k.flash.own_m { sam_secs = g.k.flash.own_secs * k; }
                        } else {
                            sam_secs = v3::lerp(g.k.flash.secs[0], g.k.flash.secs[1], u) * k;
                            stag = v3::lerp(g.k.flash.stagger[0], g.k.flash.stagger[1], u) * k;
                        }
                    }
                }
                if sam_secs > 0.0 {
                    crate::samstun::dazzle_player(g, sam_secs, at);
                    if stag > 0.0 {
                        let c = g.pl[g.player].ch;
                        let dir = v3::sub(g.ch[c].pos, at);
                        crate::samstun::stagger_player(g, stag, dir);
                    }
                }
            }
        }
        if !told {
            told = true;
            let text: String = if side == crate::teams::Team::Hostile {
                if n != 0 { format!("their stun charge — {} dazzled", n) } else { String::from("their stun charge went off") }
            } else if n != 0 {
                format!("flashbang — {} guard{} dazzled", n, if n > 1 { "s" } else { "" })
            } else {
                String::from("flashbang went off")
            };
            crate::game::game_msg(g, &text);
        }
        if sam_secs > 0.0 {
            let by = if sam_side == crate::teams::Team::Player || sam_side == crate::teams::Team::Operative { "a flashbang" } else { "their stun charge" };
            let text = format!("{} blinded you", by);
            crate::game::game_msg(g, &text);
        }
    });
}

/// Sandbox / panel: an armed can dropped at `p` as if just landed, thrown from where the player stands.   (combat.ts dropThrowable)
pub fn drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) {
    let c = g.pl[g.player].ch;
    let from = g.ch[c].pos;
    let heading = v3::yaw_to(from, p);
    let fuse = if kind == crate::game::ThrowKind::Frag { g.k.frag.fuse } else { 0.35 };
    let item_kind = match kind {
        crate::game::ThrowKind::Smoke => crate::throwables::ItemKind::Smoke,
        crate::game::ThrowKind::Flash => crate::throwables::ItemKind::Flash,
        crate::game::ThrowKind::Frag => crate::throwables::ItemKind::Frag,
    };
    let it = crate::throwables::throwables_spawn(
        g,
        item_kind,
        [p[0], m::max(p[1], 0.0) + 0.25, p[2]],
        [m::sin(heading) * 0.5, 0.0, m::cos(heading) * 0.5],
        Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(fuse), heading: Some(heading), ..Default::default() }),
    );
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
}

/// g.col.segmentBlocked(a, b) with its defaults.
fn seg(g: &mut Game, a: V3, b: V3) -> bool {
    crate::scene_collision::segment_blocked(g, a, b, None, None, None)
}

/// eachSeat(g, f) over one view: seats::each_seat's closure takes two `&mut Game` and cannot be lent, so its one body is here, through with_seat.
fn on_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    if g.seats.len() == 1 { f(g); return; }
    let mut i = 0;
    while i < g.seats.len() {
        let s = g.seats[i];
        i += 1;
        let man = crate::cmd::seat_man(g, s);
        crate::seats::with_seat(g, man, &mut *f);
    }
}

/// light.radius = r: a LightId has no table and no accessor, and RtLight is nowhere in the index.
fn seam_light_set_radius(g: &mut Game, light: LightId, r: f64) {
    todo!("needs: a setter of a Light's radius (combat.ts:691)")
}
