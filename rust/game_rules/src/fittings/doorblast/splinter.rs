//! A leaf shot through: it goes in pieces the way the round was travelling, and the lock rail drops where it stood.
use super::*;

/// What a leaf going in pieces carries from stage to stage.
struct Splinter {
    d: DoorId,
    who: f64,
    stands: Stands,
    /// The face it goes out by, and the round's way bent toward it.
    fly: f64,
    way: [f64; 2],
    /// Its width, its height and its thickness, and half its box's width.
    w: f64,
    h: f64,
    thick: f64,
    hw: f64,
    albedo: V3,
    yaw: f64,
    seed: f64,
}

/// The lock rail: its half sizes, where its handle is on it, and how far along the leaf it was cut.
struct Rail {
    bits: [f64; 4],
    along: f64,
}

/// One piece as it is cut: its middle in the leaf's own axes and its half sizes.
struct Piece {
    lx: f64,
    ly: f64,
    sw: f64,
    sl: f64,
}

/// How one piece leaves.
struct Throw {
    at: V3,
    v: V3,
    spin: V3,
    tilt: f64,
}

/// The leaf goes in pieces the way a round travelling `dir` sends it; the lock rail stays as its box.
pub fn splinter(g: &mut Game, d: DoorId, dir: V3, who: f64, at: Option<V3>) -> bool {
    if stays(g, d) {
        return false;
    }
    let s = Splinter::of(g, d, dir, who, at);
    frame_lets_go(g, d, who);
    let pos = crate::doors::door_pos(g, d);
    let level = g.k.doorshot.level;
    crate::doors::doors_sound(g, d, crate::doors::DoorSound::Kick, pos, who, level);
    let rail = s.goes(g);
    s.bursts(g);
    let lies = s.lay(g, &rail);
    leaf_is_off(g, d, s.fly);
    g.dr[d].slab = Some(crate::doors::Slab { ang: g.dr[d].leaf.angle, ..lies });
    hands_off(g, d);
    let line = format!("{} splintered by gunfire · {} pieces so far", g.dr[d].def.name, g.stats.doorblast.doorshot_chunks);
    note(g, &line);
    pose_slab(g, d);
    s.wakes(g, lies.p, rail.bits[1]);
    crate::doors::doors_torn(g, d, who, Some(crate::wallbreak::LevelChangeCause::Fire));
    true
}

impl Splinter {
    fn of(g: &Game, d: DoorId, dir: V3, who: f64, at: Option<V3>) -> Splinter {
        let stands = stands(g, d);
        let [ux, uz] = stands.u;
        let fly: f64 = if dir[2] * ux - dir[0] * uz >= 0.0 { 1.0 } else { -1.0 };
        let row = &g.tab.boxes[g.dr[d].box_];
        Splinter {
            d,
            who,
            fly,
            way: way_out(dir, stands.out(fly)),
            w: g.dr[d].def.width,
            h: g.dr[d].height,
            thick: g.dr[d].thick,
            hw: g.dr[d].leaf_h[0],
            albedo: row.albedo,
            yaw: row.yaw,
            seed: seed_of(g, d, &stands, at),
            stands,
        }
    }

    /// Every piece but the lock rail leaves, as a chunk while there are any to give and as a burst of slivers after.
    fn goes(&self, g: &mut Game) -> Rail {
        let (w, h) = (self.w, self.h);
        let k = g.k.doorshot.clone();
        let [handle_x, handle_y] = handle_on(w, h);
        let xs = cuts(self.seed, m::max(1.0, k.strips), 10.0, k.uneven);
        let mut rail: Option<Rail> = None;
        let mut left = k.chunks;
        let mut nth = 100.0;
        for (i, x) in xs.windows(2).enumerate() {
            let ys = cuts(self.seed, m::max(1.0, k.lengths), 20.0 + (i as f64) * 8.0, k.uneven);
            for y in ys.windows(2) {
                let p = Piece {
                    lx: (x[0] + x[1]) / 2.0 * self.hw,
                    ly: (y[0] + y[1]) / 2.0 * h / 2.0,
                    sw: (x[1] - x[0]) / 2.0 * self.hw,
                    sl: (y[1] - y[0]) / 2.0 * h / 2.0,
                };
                if rail.is_none() && m::abs(handle_x - p.lx) <= p.sw && m::abs(handle_y - p.ly) <= p.sl {
                    rail = Some(p.as_rail(handle_x, handle_y));
                    continue;
                }
                let t = self.thrown(&k, &p, [0.0, 1.0, 2.0].map(|n| r_of(self.seed, nth + n)));
                nth += 3.0;
                let chunk = if left > 0.0 { self.chunk(g, &p, &t) } else { None };
                left -= 1.0;
                if chunk.is_none() {
                    let o = chips(self.who, self.albedo, 2.4, 1.1, 18.0);
                    debris_burst(g, t.at, [t.v[0], m::max(0.3, t.v[1]), t.v[2]], 14.0, o);
                }
            }
        }
        rail.unwrap_or(Rail { bits: [self.hw / m::max(1.0, k.strips), h / 2.0 / m::max(1.0, k.lengths), 0.0, 0.0], along: 0.0 })
    }

    /// How a piece leaves, by its own three numbers: out with the round, and a little to one side; one from near an edge goes in toward the middle.
    fn thrown(&self, k: &crate::knobs::DoorshotK, p: &Piece, r: [f64; 3]) -> Throw {
        let ([ux, uz], [dx, dz]) = (self.stands.u, self.way);
        let speed = v3::lerp(k.fly[0], k.fly[1], r[0]);
        let mut aside = (r[1] - 0.5) * 2.0 * k.side;
        if aside * p.lx > 0.0 && m::abs(p.lx) > self.hw * 0.4 {
            aside *= -0.5;
        }
        let top = self.fly * v3::lerp(k.topple[0], k.topple[1], r[1]);
        Throw {
            at: [self.stands.c[0] + ux * p.lx, self.h / 2.0 + 0.01 + p.ly, self.stands.c[1] + uz * p.lx],
            v: [dx * speed + ux * aside, v3::lerp(k.up[0], k.up[1], r[2]), dz * speed + uz * aside],
            spin: [(r[0] - 0.5) * k.spin + ux * top, (r[1] - 0.5) * 0.6 * k.spin, (r[2] - 0.5) * k.spin + uz * top],
            tilt: self.fly * v3::lerp(k.tilt[0], k.tilt[1], r[2]),
        }
    }

    /// The piece as a chunk of wood, if the pile has room for one.
    fn chunk(&self, g: &mut Game, p: &Piece, t: &Throw) -> Option<crate::ids::DebrisChunkId> {
        let [ux, uz] = self.stands.u;
        let rot = v3::quat::mul(v3::quat::axis_angle([ux, 0.0, uz], t.tilt), v3::quat::yaw(self.yaw));
        let f = crate::scene_boxes::copy_box(g, g.dr[self.d].box_);
        {
            let mut piece = g.tab.boxes.edit(f);
            piece.c = t.at;
            piece.h = [p.sw, p.sl, self.thick / 2.0];
            piece.yaw = self.yaw;
            piece.rot = Some(rot);
            piece.marks = None;
        }
        let land = crate::furnblast::furn_break_row(crate::scene_materials::BreakClass::Wood).land;
        let c = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, f, t.v, t.spin, Some(self.who), Some(land.level))?;
        g.stats.doorblast.doorshot_chunks += 1;
        g.tab.debris_chunk[c].level = land.level;
        g.tab.debris_chunk[c].sfx = land.sfx.iter().map(|s| crate::debris::ChunkSfxItem { name: s.name.clone(), rate: s.rate, gain: s.gain }).collect();
        crate::break_table::breaks_retag(g, c, crate::break_table::Mat::Wood);
        crate::break_table::breaks_dress(g, c);
        Some(c)
    }

    /// The slivers the round takes with it, and the chips out of both faces.
    fn bursts(&self, g: &mut Game) {
        let ([cx, cz], [dx, dz], h) = (self.stands.c, self.way, self.h);
        let k = g.k.doorshot.clone();
        let slivers = crate::render_particles::EmitOpts {
            speed: k.slivers[1],
            speed_jit: Some(0.6),
            spread: Some(0.9),
            life: 20.0,
            life_jit: Some(0.3),
            size: 0.028,
            size_jit: Some(0.5),
            aspect: Some(5.0),
            color: Some(v3::copy(self.albedo)),
            owner: Some(self.who),
            ..Default::default()
        };
        debris_burst(g, [cx, h * 0.5, cz], [dx, 0.3, dz], k.slivers[0], slivers);
        for (s, n, speed) in [(1.0, k.chips[0], k.chip_v[0]), (-1.0, k.chips[1], k.chip_v[1])] {
            let o = chips(self.who, self.albedo, speed, 1.1, 18.0);
            debris_burst(g, [cx, h * 0.55, cz], [dx * s, 0.25, dz * s], n, o);
        }
    }

    /// The lock rail as it lies: where it dropped, a little the round's way, clear of every static, or else in its own doorway.
    fn lay(&self, g: &Game, rail: &Rail) -> crate::doors::Slab {
        let ([ux, uz], [cx, cz], [dx, dz]) = (self.stands.u, self.stands.c, self.way);
        let [hu, hn] = [rail.bits[0], rail.bits[1]];
        let (y0, y1) = (0.06, 0.3);
        let mut a = self.stands.a + (r_of(self.seed, 7.0) - 0.5) * 0.8;
        let mut at = [cx + ux * rail.along + dx * 0.3, cz + uz * rail.along + dz * 0.3];
        if overlaps(g, at[0], at[1], a, hu, hn, y0, y1) {
            match free_spot(g, at[0], at[1], a, hu, hn, y0, y1, 0.9) {
                Some(to) => at = to,
                None => {
                    let fc = g.dr[self.d].frame_centre;
                    at = [fc[0], fc[2]];
                    a = g.dr[self.d].def.closed_dir;
                }
            }
        }
        crate::doors::Slab {
            p: [at[0], self.thick / 2.0 + LIFT, at[1]],
            v: [0.0, 0.0, 0.0],
            a,
            spin: 0.0,
            tip: HALF_PI,
            tip_v: 0.0,
            side: self.fly,
            rest: true,
            landed: true,
            who: self.who,
            t: 0.0,
            ang: 0.0,
            bits: Some(rail.bits),
            wedged: 0.0,
        }
    }

    /// A body asleep against the leaf, or where the rail came down, slides now that it is gone.
    fn wakes(&self, g: &mut Game, rail_at: V3, rail_l: f64) {
        let [hx, hz] = g.dr[self.d].def.hinge;
        let mut chars = vec![g.pl[g.player].ch];
        chars.extend(g.guards.iter().map(|o| g.gd[*o].ch));
        chars.extend(crate::civilians::civilians_shown(g).into_iter().map(|o| g.gd[o].ch));
        for ch in chars {
            let Some(rag) = g.ch[ch].ragdoll.filter(|rag| g.tab.ragdoll[*rag].sleeping) else { continue };
            let h = g.ch[ch].bone(crate::character::Bone::Hips).unwrap_or(g.ch[ch].pos);
            if m::hyp2(h[0] - hx, h[2] - hz) < self.w + 1.2 || crate::v3::dist_xz(h, rail_at) < rail_l + 0.8 {
                crate::ragdoll::ragdoll_wake(g, rag);
            }
        }
    }
}

impl Piece {
    fn as_rail(&self, handle_x: f64, handle_y: f64) -> Rail {
        let mx = m::max(0.0, self.sw - 0.065);
        let my = m::max(0.0, self.sl - 0.03);
        Rail { bits: [self.sw, self.sl, v3::clamp(handle_x - self.lx, -mx, mx), v3::clamp(handle_y - self.ly, -my, my)], along: self.lx }
    }
}

/// The floor heading the pieces take: the round's, bent toward the face it goes out by.
fn way_out(dir: V3, [nx, nz]: [f64; 2]) -> [f64; 2] {
    let l = m::hyp2(dir[0], dir[2]);
    if l < 0.2 {
        return [nx, nz];
    }
    let (dx, dz) = (0.6 * dir[0] / l + 0.4 * nx, 0.6 * dir[2] / l + 0.4 * nz);
    let l = m::hyp2(dx, dz);
    [dx / l, dz / l]
}

/// The same door struck in the same place goes to the same pieces.
fn seed_of(g: &Game, d: DoorId, stands: &Stands, at: Option<V3>) -> f64 {
    let [hx, hz] = g.dr[d].def.hinge;
    let struck = match at {
        Some(at) => m::floor(((at[0] - hx) * stands.u[0] + (at[2] - hz) * stands.u[1]) * 16.0) * 7.0 + m::floor(at[1] * 16.0) * 31.0,
        None => 0.0,
    };
    let nth = g.doors.list.iter().position(|x| *x == d).map_or(-1.0, |i| i as f64);
    nth * 977.0 + 131.0 + struck
}

/// `R(k)` of a leaf's seed: the hash of (seed + k).
fn r_of(seed: f64, k: f64) -> f64 {
    crate::debris::hash01((seed + k) * 2654435761.0)
}

/// The cuts of a leaf's strips or lengths, uneven by `uneven`.
fn cuts(seed: f64, n: f64, salt: f64, uneven: f64) -> Vec<f64> {
    let mut c = vec![-1.0];
    let mut j = 1.0;
    while j < n {
        c.push(-1.0 + 2.0 * (j + (r_of(seed, salt + j) - 0.5) * uneven) / n);
        j += 1.0;
    }
    c.push(1.0);
    c
}
