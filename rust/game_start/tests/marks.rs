//! THE MARKS ON THE WORLD (game::render_decals set.rs, and what the picture is told of them: rust/game_draw marks.rs). They are the game's: a game put back shows the marks it had, what
//! happened after it un-happens by itself, and nothing has to be told to anyone for that. No rule reads one: `hold` holds the rest of the game to that.
use game::render_decals::{decals_add, decals_clear, mark_set, marks_settle, DecalKind, DecalOpts};
use game_draw::{want, Drawn};
use game_start::play::{Opts, Play};

fn office(seed: u32) -> Play { Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() }) }
/// the buffer's parts, in words: the header, the floor's cells, the lists, the records
const CELL0: usize = 4;
const LIST0: usize = CELL0 + 40 * 28;
const REC0: usize = LIST0 + 4096;
fn drawn(p: &Play) -> Drawn { let mut d = Drawn::default(); d.take(&p.g, want::boxes | want::decals); d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]); d }
fn words(d: &Drawn) -> Vec<u32> { d.marks.words.iter().map(|w| w.w).collect() }
fn count(p: &Play) -> usize { mark_set(&p.g).map_or(0, |s| s.list.len()) }
fn floor_mark(p: &mut Play, kind: DecalKind, at: [f64; 2], o: DecalOpts) { decals_add(&mut p.g, kind, [at[0], 0.0, at[1]], [0.0, 1.0, 0.0], None, Some(o)); }
fn f(w: u32) -> f64 { f32::from_bits(w) as f64 }

/// THE RULE: a blast leaves marks, and a game put back shows the marks it had: word for word, with nobody told, and the keys say which is which.
/// A FAILURE MEANS: the marks are not the game's, or a tick is not what makes them.
#[test]
fn a_game_put_back_shows_the_marks_it_had() {
    let mut p = office(5);
    p.run(0.5);
    let at = p.sam_at();
    let before = p.g.clone();
    let was = words(&drawn(&p));
    game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Frag, [at[0] + 3.0, 0.0, at[1]]);
    p.run(4.0);
    let d = drawn(&p);
    let n = count(&p);
    assert!(n > 0, "a frag went off, and it left no mark");
    assert_eq!(d.marks.head[0].count as usize, n);
    assert_eq!(d.marks.keys.len(), n);
    let mut seen: Vec<_> = d.marks.keys.iter().map(|k| (k.tick, k.made)).collect();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), n, "two marks under one key");
    // (a clone holds them: the game a moment later, and the game before)
    let kept = p.g.clone();
    p.run(0.2);
    let mut back = Drawn::default();
    back.take(&kept, want::boxes | want::decals);
    back.frame(&kept, 1.0, [20.0, 0.0, 14.0]);
    assert_eq!(words(&back)[..REC0 + n * 12], words(&d)[..REC0 + n * 12], "the kept game's marks are another than it had");
    back.cut();
    back.take(&before, want::boxes | want::decals);
    back.frame(&before, 1.0, [20.0, 0.0, 14.0]);
    assert_eq!(back.marks.head[0].moved, 1);
    let listed = back.marks.head[0].listed as usize;
    assert_eq!(words(&back)[..LIST0 + listed], was[..LIST0 + listed], "a game from before the blast is drawn with what the blast left");
}

/// THE RULE: 512 marks at most. Past it the oldest goes, one laid only for the picture first; a full list of play's own takes none laid only for the picture.
#[test]
fn the_list_holds_what_it_holds_and_the_picture_s_own_go_first() {
    let mut p = office(6);
    let plain = DecalOpts::default();
    let look = DecalOpts { look: Some(true), ..Default::default() };
    floor_mark(&mut p, DecalKind::Scorch, [5.0, 5.0], look);
    for i in 0..511 { floor_mark(&mut p, DecalKind::Chip, [(i % 30) as f64 + 1.0, 6.0], plain); }
    marks_settle(&mut p.g);
    assert_eq!(count(&p), 512);
    floor_mark(&mut p, DecalKind::Chip, [7.0, 7.0], plain);
    marks_settle(&mut p.g);
    let s = mark_set(&p.g).unwrap();
    assert_eq!(s.list.len(), 512);
    assert!(s.list.iter().all(|d| !d.look), "the mark laid only for the picture is still there, and one of play's went");
    let first = s.list[0].serial;
    floor_mark(&mut p, DecalKind::Scorch, [8.0, 8.0], look);
    marks_settle(&mut p.g);
    assert_eq!(mark_set(&p.g).unwrap().list[0].serial, first, "a mark laid only for the picture pushed out one of play's");
    floor_mark(&mut p, DecalKind::Chip, [9.0, 9.0], plain);
    marks_settle(&mut p.g);
    assert_ne!(mark_set(&p.g).unwrap().list[0].serial, first, "past the cap the oldest goes");
    floor_mark(&mut p, DecalKind::Chip, [1.0, 1.0], plain);   // (asked, and then the level is put back)
    decals_clear(&mut p.g);
    marks_settle(&mut p.g);
    assert_eq!(count(&p), 0, "a clear left marks, or what was asked before it");
}

/// THE RULE: a wet print fades by itself over the time it lasts, and goes: its strength is worked out when it is drawn, from when it was made.
#[test]
fn a_wet_print_dries_by_itself() {
    let mut p = office(7);
    p.g.time = 100.0;
    let fade = p.g.k.wet.print_fade_secs;
    assert!(fade >= 2.0);
    floor_mark(&mut p, DecalKind::Wetprint, [5.0, 5.0], DecalOpts { param: Some(0.8), look: Some(true), seed: Some(1.0), ..Default::default() });
    floor_mark(&mut p, DecalKind::Wetprint, [6.0, 5.0], DecalOpts { param: Some(0.8), look: Some(true), seed: Some(0.0), ..Default::default() });
    marks_settle(&mut p.g);
    let rec = |d: &Drawn, i: usize, k: usize| d.marks.words[REC0 + i * 12 + k].w;
    let d = drawn(&p);
    assert!((f(rec(&d, 0, 7)) - 0.8).abs() < 1e-6, "as made: {}", f(rec(&d, 0, 7)));
    assert_eq!(rec(&d, 0, 11) >> 4 & 1, 1, "which foot");
    assert_eq!(rec(&d, 1, 11) >> 4 & 0xfff, 36, "the next print has the next look");
    p.g.time += fade / 2.0;
    marks_settle(&mut p.g);
    let d = drawn(&p);
    assert!((f(rec(&d, 0, 7)) - 0.4).abs() < 0.02, "half way through its life: {}", f(rec(&d, 0, 7)));
    p.g.time += fade;
    marks_settle(&mut p.g);
    assert_eq!(count(&p), 0, "a dry print is still there");
}

/// THE RULE: a mark on a leaf rides it: drawn where the leaf is drawn this frame, on that box alone, and not at all while the box is not in the list.
#[test]
fn a_mark_on_a_leaf_rides_it() {
    let mut p = office(8);
    p.run(0.1);
    let door = p.g.doors.list[0];
    let leaf = p.g.dr[door].box_;
    let c = p.g.tab.boxes[leaf].c;
    game::marks::kick_scuff(&mut p.g, door, [c[0] + 0.5, 0.0, c[2] + 0.5]);
    marks_settle(&mut p.g);
    assert_eq!(count(&p), 1, "a kicked leaf took no scuff");
    let d = drawn(&p);
    let bits = d.marks.words[REC0 + 11].w;
    assert_eq!(bits & 8, 8, "said to ride");
    let on = (bits >> 16) as usize;
    let g = d.boxes.geo[on].w;
    let centre = [f32::from_bits(g[0]), f32::from_bits(g[1]), f32::from_bits(g[2])];
    assert_eq!(centre, [c[0] as f32, c[1] as f32, c[2] as f32], "box {on} of the list is not the leaf");
    let was = [f(d.marks.words[REC0].w), f(d.marks.words[REC0 + 2].w)];
    p.g.tab.boxes[leaf].c[0] += 1.0;
    let d = drawn(&p);
    assert!((f(d.marks.words[REC0].w) - was[0] - 1.0).abs() < 1e-5 && (f(d.marks.words[REC0 + 2].w) - was[1]).abs() < 1e-5, "the mark stayed where the leaf was");
    p.g.engine.world.dynamics.retain(|b| *b != leaf.into());
    let d = drawn(&p);
    assert_eq!((d.marks.head[0].count, d.marks.head[0].drawn, f(d.marks.words[REC0 + 3].w)), (1, 0, 0.0), "drawn with its box gone");
}

/// THE RULE: a round counted on a wall makes its own bullet mark the wall's wear, sized by how far gone the cell is; the marks the cell already has grow with it; a thin wall's goes on its mid-plane.
#[test]
fn wear_makes_the_round_s_own_mark_and_grows_with_the_cell() {
    use game::scene_materials::Surf;
    let mut p = office(9);
    let wall = p.g.col.boxes.iter().copied().find(|b| { let x = &p.g.tab.boxes[*b]; x.h[0] > 0.5 && x.h[1] > 0.5 && x.h[2] >= 0.04 && x.h[2] <= 0.09 && x.yaw == 0.0 && x.rot.is_none() }).expect("a thin wall");
    let (c, h) = (p.g.tab.boxes[wall].c, p.g.tab.boxes[wall].h);
    let hit = [c[0], c[1], c[2] + h[2]];
    decals_add(&mut p.g, DecalKind::Bullet, hit, [0.0, 0.0, 1.0], Some(0.031), Some(DecalOpts { seed: Some(0.0), ..Default::default() }));
    game::wallwear::wall_wear_round(&mut p.g, wall, Surf::Wood, 7.0, 7.0, 0.1, hit, [0.0, 0.0, -1.0]);
    marks_settle(&mut p.g);
    let d0 = mark_set(&p.g).unwrap().list[0];
    assert_eq!(d0.kind, DecalKind::Damage);
    assert!((d0.pos[2] - c[2]).abs() < 1e-9, "a thin wall's mark lies on its mid-plane: {:?}", d0.pos);
    assert_eq!(d0.param.floor(), 4.0, "wear's look and wood");
    decals_add(&mut p.g, DecalKind::Bullet, [hit[0] + 0.3, hit[1], hit[2]], [0.0, 0.0, 1.0], Some(0.031), None);
    game::wallwear::wall_wear_round(&mut p.g, wall, Surf::Wood, 7.0, 7.0, 0.6, [hit[0] + 0.3, hit[1], hit[2]], [0.0, 0.0, -1.0]);
    marks_settle(&mut p.g);
    let s = mark_set(&p.g).unwrap();
    assert!(s.list[0].param > d0.param && s.list[0].radius > d0.radius, "the first mark of the cell did not grow: {} {}", s.list[0].param, d0.param);
}

/// THE RULE: the buffer says a mark in every metre its bounding circle overlaps, the newest 48 of a metre, and the page is told when a word of it moved.
#[test]
fn the_buffer_is_as_the_shader_reads_it() {
    let mut p = office(10);
    floor_mark(&mut p, DecalKind::Scorch, [10.5, 10.5], DecalOpts::default());
    marks_settle(&mut p.g);
    let mut d = Drawn::default();
    d.take(&p.g, want::boxes | want::decals);
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    assert_eq!(d.marks.head[0].moved, 1, "the first frame tells it");
    let w = words(&d);
    let cell = |x: usize, z: usize| w[CELL0 + z * 40 + x];
    assert_eq!(cell(10, 10) >> 16, 1);
    assert_eq!(w[LIST0 + (cell(10, 10) & 0xffff) as usize], 0);
    assert_eq!(cell(9, 10) >> 16, 1, "0.75 m round 10.5 reaches the next metre");
    assert_eq!(cell(13, 10) >> 16, 0);
    assert_eq!(&w[..2], &[1, d.marks.head[0].listed]);
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    assert_eq!(d.marks.head[0].moved, 0, "nothing moved, and the page is told to send it");
    for i in 0..60 { floor_mark(&mut p, DecalKind::Chip, [20.5, 20.5 + i as f64 * 1e-3], DecalOpts::default()); }
    marks_settle(&mut p.g);
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    let w = words(&d);
    let c = w[CELL0 + 20 * 40 + 20];
    assert_eq!(c >> 16, 48, "a metre's newest");
    assert_eq!(w[LIST0 + (c & 0xffff) as usize + 47], 60, "…ending in the newest mark");
    assert_eq!(d.marks.head[0].moved, 1);
}

/// A charge on the office's partition (tests/wallcharge.rs has the place), on a game with a particle system, as a page's has.
fn a_charge_set(seed: u32) -> (Play, game::ids::WallChargeId) {
    let mut p = office(seed);
    p.sam_unhurt();
    p.g.engine.particles = Some(game::render_particles::Particles { collide: true, spark_bounce: true, ..game::render_particles::Particles::blank() });
    p.g.engine.settings.gpu_particles = true;
    p.g.m.dress.dress.blast_feel = true;
    p.run(0.1);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    p.sam_put([14.6, 15.2]);
    p.frame();
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, [12.06, 0.0, 13.78], None) { panic!("the wall charge was refused: {why}"); }
    assert!(p.until(60.0, |p| !p.g.wall_charge.set.is_empty()).is_some(), "no charge is on the wall");
    let c = p.g.wall_charge.set[0];
    p.sam_put([20.0, 15.2]);
    p.frame();
    (p, c)
}

/// THE RULE: a wall charge's blast is told as the tick's particles, each with its id: the fireball in its own tick, the ring and the pushes and the ceiling's dust as their times come, and then nothing; and it leaves its dust on the floor.
/// A FAILURE MEANS: the picture's half of the feel has gone quiet, or a stage is played twice.
#[test]
fn a_wall_charge_is_told_as_its_stages_come() {
    let (mut p, c) = a_charge_set(11);
    let mut d = Drawn::default();
    game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
    let (mut seen, mut sparks, mut dust, mut kicks) = (Vec::new(), 0, 0, 0);
    let mut last_busy = 0;
    for k in 0..240 {
        p.frame();
        d.take(&p.g, want::events);
        for e in &d.events.emits { assert!(!seen.contains(&(e.tick, e.serial)), "an id told twice"); seen.push((e.tick, e.serial)); if e.seed != u32::MAX { last_busy = k; if e.kind == game::render_particles::PKind::Spark as u32 { sparks += 1; } if e.kind == game::render_particles::PKind::Dust as u32 { dust += 1; } } }
        kicks += d.events.kicks.iter().filter(|e| e.seed != u32::MAX).count();
    }
    assert!(sparks >= 2 && dust > 20, "the fireball and the ring: {sparks} lots of sparks, {dust} of dust");
    assert!(kicks >= 3, "the three pushes: {kicks}");
    assert!(last_busy < 200, "its stages are over within a few seconds, and something was still told at tick {last_busy}");
    assert!(mark_set(&p.g).unwrap().list.iter().any(|m| m.look && m.kind == DecalKind::Damage), "no film of dust where the wall went");
}

/// THE RULE: the level put back, nothing of the old encounter's blast plays into this one.
#[test]
fn a_level_put_back_stops_a_blast_s_stages() {
    let (mut p, c) = a_charge_set(12);
    game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
    p.frame();
    decals_clear(&mut p.g);
    let mut d = Drawn::default();
    p.frame();
    d.take(&p.g, want::events);
    let told = |d: &Drawn| d.events.emits.iter().filter(|e| e.seed != u32::MAX).count();
    for _ in 0..60 { p.frame(); d.take(&p.g, want::events); assert_eq!(told(&d), 0, "a stage came after the level was put back"); }
}

/// THE RULE: the traffic map is one word for two texels over the floor; where a door stands the floor is walked; along a long wall stones gather, and a level's own map does not change as it is played.
#[test]
fn the_traffic_map_says_where_the_level_is_walked() {
    let mut p = office(13);
    let mut d = Drawn::default();
    d.take(&p.g, want::level_look);
    assert_eq!(d.traffic.words.len(), 40 * 8 * 28 * 8 / 2);
    let texel = |d: &Drawn, i: usize| (d.traffic.words[i / 2].w >> (i % 2 * 16)) & 0xffff;
    let at = |x: f64, z: f64| (z * 8.0) as usize * 320 + (x * 8.0) as usize;
    let door = &p.g.level.doors[0];
    let mid = [door.hinge[0] + door.closed_dir.cos() * door.width * 0.5, door.hinge[1] + door.closed_dir.sin() * door.width * 0.5];
    assert!(texel(&d, at(mid[0], mid[1])) & 0xff > 100, "a door's threshold is not worn: {}", texel(&d, at(mid[0], mid[1])) & 0xff);
    assert!((0..320 * 224).any(|i| texel(&d, i) >> 8 == 255), "no band along any wall");
    assert!((0..320 * 224).filter(|&i| texel(&d, i) & 0xff == 0).count() > 320 * 224 / 2, "the whole floor is walked");
    let was = d.traffic.words.clone();
    p.run(0.5);
    d.take(&p.g, want::level_look);
    assert!(was == d.traffic.words, "the map moved");
}

/// THE RULE: on a level that wears the look a leaf stands in its paint, a shade of its own by its name, on the game's own box, so what is drawn and what a breach throws off it are in that paint.
#[test]
fn a_leaf_wears_the_level_s_paint_from_its_first_moment() {
    let mut p = Play::mission("compound-gwot.json", "?levelFile=compound-gwot.json&roomclear&squad4", &Opts { seed: 14, dress: true, ..Default::default() });
    p.run(0.1);
    let list = p.g.doors.list.clone();
    let (office_in, office_out) = ([0.42, 0.3, 0.19], [0.2, 0.22, 0.25]);
    let leaves: Vec<_> = list.iter().copied().filter(|d| p.g.dr[*d].def.steel != Some(true)).collect();
    assert!(!leaves.is_empty(), "the compound has doors");
    for d in &leaves { let a = p.g.tab.boxes[p.g.dr[*d].box_].albedo; assert!(a != office_in && a != office_out, "a leaf stood in the office's colours: {a:?}"); }
    // what is drawn is the box
    let mut d = Drawn::default();
    d.take(&p.g, want::boxes);
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    let n = p.g.engine.world.statics.len();
    let drawn = &p.g.engine.world.dynamics;
    for l in &leaves {
        let b = p.g.dr[*l].box_;
        let k = drawn.iter().position(|x| *x == game::ids::AnyBoxId::from(b)).expect("a leaf is in the list");
        let (m, a) = (&d.boxes.mat[n + k].w, p.g.tab.boxes[b].albedo);
        assert!((0..3).all(|i| (m[i] as f64 - a[i]).abs() < 1e-5), "drawn in {:?}, the box is {a:?}", &m[..3]);
    }
    // and what a breach throws off a leaf is in that paint
    let l = leaves[0];
    let (paint, word) = { let b = &p.g.tab.boxes[p.g.dr[l].box_]; (b.albedo, b.stuff) };
    let before = p.g.destr.debris.list.len();
    assert!(game::doorblast::splinter(&mut p.g, l, [1.0, 0.0, 0.0], 0.0, None), "the leaf would not splinter");
    let thrown: Vec<_> = p.g.destr.debris.list[before..].to_vec();
    assert!(!thrown.is_empty(), "a splintered leaf threw nothing");
    // (a dressed piece's colour is its paint tinted to the break's raw core: the paint it broke off in is the chunk's record)
    for c in thrown {
        let ch = &p.g.tab.debris_chunk[c];
        assert_eq!(ch.mat.as_ref().expect("a leaf's piece is tagged wood").paint, paint, "a piece broke off in another paint than the leaf's");
        assert_eq!(p.g.tab[ch.box_].stuff, word, "a piece is made of something else than the leaf");
    }
}

/// … and a game stood on an address that says the look is off keeps the leaves as they are made (`look=plain`: scenario_run::address_before_game).
#[test]
fn the_look_off_paints_nothing() {
    let on = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &Opts { seed: 14, ..Default::default() });
    let leaf = |p: &Play, d| (p.g.tab.boxes[p.g.dr[d].box_].albedo, p.g.dr[d].def.exterior == Some(true), p.g.dr[d].def.steel == Some(true));
    let plain = |a: [f64; 3], out: bool| a == if out { [0.2, 0.22, 0.25] } else { [0.42, 0.3, 0.19] };
    assert!(on.g.doors.list.iter().any(|d| { let (a, out, steel) = leaf(&on, *d); !steel && !plain(a, out) }), "the compound wears the look, and no leaf is painted");
    drop(on);
    let off = Play::mission("compound.json", "?levelFile=compound.json&look=plain&roomclear&squad4", &Opts { seed: 14, ..Default::default() });
    assert!(!off.g.k.look.on, "the address said the look is off");
    for d in &off.g.doors.list { let (a, out, steel) = leaf(&off, *d); assert!(steel || plain(a, out), "the look is off, and a leaf is in {a:?}"); }
}
