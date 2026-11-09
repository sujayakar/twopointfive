//! src/game/combat.ts lines 664 to 757
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

/// Seconds a stun can sits after it lands: FLASH.fuse in a room fight, else the night game's.   (combat.ts flashFuse)
pub fn flash_fuse(g: &crate::game::Game) -> f64 {
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
fn detonate_stun_raw(g: &mut crate::game::Game, it: ItemId, borrowed: bool, sam: Option<crate::combat::DetonateStunRawSam>) {
    let b = crate::effects::bang();
    let p = crate::throwables::item_pos(g, it);
    let at: V3 = [p[0], p[1] + b.height, p[2]];
    let tear = g.k.doorblast.tear_m.stun;
    let who = crate::throwables::item_who(g, it);
    crate::doors::doors_blast(g, at, tear, who);
    let seed = g.fx_seed;
    g.fx_seed += 1.0;
    let o = crate::effects::SpawnBurstO { spark_count: Some(m::min(b.spark_count, if crate::effects::flags().trails { b.trail_count } else { 0.0 })) };
    let l = crate::effects::spawn_burst(g, at, seed, Some(o));
    crate::particle_fx::bang_fx(g, at);
    let flash = crate::render_lights::light_set_flash(g, l.pos, l.color, l.intensity, l.range, l.ttl, 0.0);
    let light = crate::fogofwar::fog_of_war_light_by(g, flash, Some(who));
    if let Some(r) = l.radius {
        if js::truthy(r) { g.tab.light[light].radius = r; }
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
        if !crate::condition::down(g, g.pl[ears].ch) && v3::dist(eye, at) < b.dazzle_radius && !seg(g, v3::add(at, [0.0, 0.3, 0.0]), eye) {
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
        ..crate::game::GameEvent::default()
    });
    let side: crate::teams::Team = match g.tab.item[it].side {
        Some(s) => s,
        None => {
            let who = crate::throwables::item_who(g, it);
            if js::truthy(who) { crate::teams::team_of_id(g, who) } else { crate::teams::Team::Player }
        }
    };
    let (from, over, entry) = (v3::add(at, [0.0, 0.3, 0.0]), v3::add(at, [0.0, g.k.flash.lift, 0.0]), crate::roomfight::fight_live(g));
    let flash = Flash { at, from, over, entry, lifted: entry && !seg(g, from, over), radius: b.dazzle_radius, secs: [b.dazzle_seconds, b.dazzle_min] };
    let squad = |t: crate::teams::Team| t == crate::teams::Team::Player || t == crate::teams::Team::Operative;
    if crate::effects::flags().dazzle {
        for gd in g.guards.clone() {
            let (c, team) = (g.gd[gd].ch, g.gd[gd].team);
            if crate::condition::down(g, c) { continue; }
            let Some((secs, fold)) = flash.takes(g, c, team == side || squad(side) && squad(team), false, false) else { continue };
            if fold > 0.0 {
                let drilled = crate::roomfight::drilled_k(g, gd);
                crate::condition::fold(g, c, fold * drilled);
                let gun = g.gd[gd].weapon;
                g.fa[gun].burst = None;
            }
            crate::condition::dazzled(g, gd, secs, at, Some(at), "combat");
            let along = v3::sub(g.ch[c].pos, at);
            crate::character::anim_hit_from(g, c, Some(along));
        }
    }
    let sam_side: crate::teams::Team = sam.and_then(|s| s.side).unwrap_or(side);
    let spare = sam.and_then(|s| s.spare).unwrap_or(false);
    crate::seats::each_seat(g, &mut |g: &mut Game| {
        let c = g.pl[g.player].ch;
        if borrowed || !crate::effects::flags().dazzle || crate::condition::down(g, g.pl[g.player].ch) || g.puppet.is_some() { return; }
        let Some((secs, fold)) = flash.takes(g, c, squad(sam_side), spare, g.k.samstun.own_seen) else { return };
        if !(secs > 0.0) { return; }
        crate::samstun::dazzle_player(g, secs, at);
        if fold > 0.0 {
            let dir = v3::sub(g.ch[c].pos, at);
            crate::samstun::stagger_player(g, fold, dir);
        }
    });
}

/// a flash as it goes off: where, the two places its light comes from, in a room fight or not, its reach and its seconds
struct Flash { at: V3, from: V3, over: V3, entry: bool, lifted: bool, radius: f64, secs: [f64; 2] }
impl Flash {
    /// What it does to this man: the seconds blind and the seconds folded. `own`: his own side's; `spare`: thrown for him to come in behind; `own_seen`: his own side's
    /// takes him as any other does where he looks at it
    fn takes(&self, g: &mut Game, c: CharId, own: bool, spare: bool, own_seen: bool) -> Option<(f64, f64)> {
        let bare = if matches!(g.ch[c].who, crate::character::Who::Player(_)) { 1.6 } else { 1.65 };
        let head = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or_else(|| v3::add(g.ch[c].pos, [0.0, bare, 0.0]));
        let d = v3::dist(head, self.at);
        if d > self.radius { return None; }
        let u = v3::clamp(d / self.radius, 0.0, 1.0);
        if !self.entry {
            if spare && !own_seen || seg(g, self.from, head) { return None; }
            return Some((v3::lerp(self.secs[0], self.secs[1], u), 0.0));
        }
        let seen = !seg(g, self.from, head) || (self.lifted && !seg(g, self.over, head));
        if own && spare && !(seen && own_seen) { return None; }
        let mut k = 1.0;
        if !seen {
            let pr = crate::scene_collision::sound_paths_propagate(g, self.at, head, None);
            if pr.muffle > 0.6 || pr.path_len > g.k.flash.concuss_m { return None; }
            k = g.k.flash.concuss_k;
        }
        if own && !(seen && own_seen) { return if d <= g.k.flash.own_m { Some((g.k.flash.own_secs * k, 0.0)) } else { None }; }
        let (secs, fold) = (g.k.flash.secs, g.k.flash.stagger);
        Some((v3::lerp(secs[0], secs[1], u) * k, v3::lerp(fold[0], fold[1], u) * k))
    }
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
