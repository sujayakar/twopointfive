//! A FULL DEBRIS POOL: the rigid pool holds 64 pieces and no more. While they are all still in the air a new piece is refused; once they lie at
//! rest a new piece takes a resting piece's slot. A failure means a big blast can outgrow the pool, or that a pool once full never takes another.
use game::debris::ChunkKind;
use game_start::play::{Opts, Play};
use game_start::stand;

const DT: f64 = 1.0 / 60.0;
const CAP: usize = 64;

/// A small slab over the man's head, `i` of a grid wide enough for the whole pool.
fn slab(p: &mut Play, i: usize, up: f64) -> bool {
    let at = p.g.pl[0].char.pos;
    let c = [at[0] + 0.3 * ((i % 8) as f64 - 3.5), up, at[2] + 0.3 * ((i / 8) as f64 - 3.5)];
    let b = game::scene_boxes::make_box(&mut p.g, &game::scene_boxes::MakeBoxP { c, h: [0.05, 0.01, 0.05], ..game::scene_boxes::MakeBoxP::zeroed() });
    game::debris::debris_spawn_raw(&mut p.g, ChunkKind::Chunk, b, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], None, None).is_some()
}

fn stand_a_game() -> Play { Play::level("pass.json", false, &Opts::default()) }

#[test]
fn a_pool_full_of_pieces_in_the_air_refuses_the_next() {
    let mut p = stand_a_game();
    for i in 0..CAP { assert!(slab(&mut p, i, 2.0), "piece {i} of the pool was refused"); }
    assert_eq!(p.g.destr.debris.list.len(), CAP);
    assert!(!slab(&mut p, 0, 2.0), "a pool full of moving pieces took one more");
    assert_eq!(p.g.destr.debris.list.len(), CAP);
    assert_eq!(p.g.destr.debris.stats.refused, 1.0);
}

#[test]
fn a_pool_full_of_pieces_at_rest_gives_up_a_slot() {
    let mut p = stand_a_game();
    for i in 0..CAP { assert!(slab(&mut p, i, 0.2)); }
    for _ in 0..600 { stand::frame(&mut p.g, &p.host, DT); }
    assert_eq!(p.g.destr.debris.list.len(), CAP, "pieces left the pool while they settled");
    assert!(slab(&mut p, 0, 0.2), "a pool full of pieces at rest refused a new one");
    assert_eq!(p.g.destr.debris.list.len(), CAP);
    assert_eq!(p.g.destr.debris.stats.evicted, 1.0);
}
