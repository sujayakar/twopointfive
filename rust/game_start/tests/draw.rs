//! WHAT THE PICTURE IS TOLD (rust/game_draw), of games that stand. That a section says what the game has is each section's own test (`fog.rs` holds the fog's);
//! these hold what is true of all of them: drawing a game changes nothing of it, what is drawn between two ticks stands between them, and nothing is drawn
//! through a cut. `hold draw` draws every frame of a set of plays and says what it cost.
use game_draw::{want, Drawn};
use game_start::play::{words, Opts, Play};

const ALL: u32 = u32::MAX;
fn office(seed: u32) -> Play { Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() }) }
fn man(d: &Drawn, id: u32) -> Option<[f32; 3]> { let i = d.men.men.iter().position(|m| m.id == id)?; Some([d.men.bones[i].w[12], d.men.bones[i].w[13], d.men.bones[i].w[14]]) }
fn head(d: &Drawn) -> &game_draw::boxes::Head { &d.boxes.head[0] }

/// THE RULE: every section is made of the game shared, so the game drawn after every frame is, in every part of its state, the game nobody drew.
/// A FAILURE MEANS: a reader a section calls has come to write through the shared game (rust/sim's statics, a memo in a cell).
#[test]
fn drawing_a_game_is_no_part_of_it() {
    let played = |draw: bool| {
        let mut p = office(5);
        let mut d = Drawn::default();
        (0..240).map(|k| {
            if k == 60 { let at = p.sam_at(); game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Frag, [at[0] + 3.0, 0.0, at[1]]); }
            p.g.god_mode = true;
            p.frame();
            if draw { let focus = p.g.aim_point; d.take(&p.g, ALL); d.frame(&p.g, 0.5, focus); d.frame(&p.g, 1.0, focus); }
            words(&mut p.g)
        }).collect::<Vec<_>>()
    };
    let (plain, drawn) = (played(false), played(true));
    assert_eq!(plain.iter().zip(&drawn).position(|(a, b)| a != b), None, "the drawn game parts from the one nobody drew");
}

/// THE RULE: half way between two ticks a man stands half way between where the two left him, his proxies with him; at the whole he stands where the last left him.
#[test]
fn a_frame_between_two_ticks_stands_between_them() {
    let mut p = office(6);
    let mut d = Drawn::default();
    p.key_down("KeyW");
    p.run(0.5);
    let id = p.g.ch[p.g.pl[p.g.player].ch].id as u32;
    let focus = p.g.aim_point;
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let before = man(&d, id).expect("Sam is drawn");
    p.frame();
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let after = man(&d, id).unwrap();
    d.frame(&p.g, 0.5, focus);
    let half = man(&d, id).unwrap();
    assert!((0..3).any(|k| (after[k] - before[k]).abs() > 1e-3), "Sam walks, and his bones did not move in a tick: {before:?} {after:?}");
    for k in 0..3 { assert!((half[k] - (before[k] + after[k]) / 2.0).abs() < 1e-4, "half way {k}: {} is not between {} and {}", half[k], before[k], after[k]); }
    d.frame(&p.g, 1.0, focus);
    assert_eq!(man(&d, id).unwrap(), after, "asked again at the whole he stands where the last tick left him");
}

/// THE RULE: after a cut (a load, a rollback, another seat) the next frame between two ticks is the last tick's: nothing moves through it.
#[test]
fn nothing_is_drawn_through_a_cut() {
    let mut p = office(7);
    let mut d = Drawn::default();
    p.key_down("KeyW");
    p.run(0.5);
    let id = p.g.ch[p.g.pl[p.g.player].ch].id as u32;
    let focus = p.g.aim_point;
    d.take(&p.g, ALL);
    p.frame();
    d.cut();
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let whole = man(&d, id).unwrap();
    d.frame(&p.g, 0.5, focus);
    assert_eq!(man(&d, id).unwrap(), whole, "he is drawn between a tick before the cut and the one after it");
}

/// THE RULE: the boxes' lists agree with their head, every static is told as changed once, and a frame whose boxes stand still keeps its grid.
#[test]
fn the_boxes_say_what_they_hold() {
    let mut p = office(5);
    let mut d = Drawn::default();
    p.run(0.2);
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    let n = p.g.engine.world.statics.len() as u32;
    assert_eq!((head(&d).statics, head(&d).from, head(&d).to), (n, 0, n), "the first frame tells every static");
    assert_eq!(head(&d).count as usize, d.boxes.geo.len());
    assert_eq!((head(&d).visible + head(&d).visible_cut) as usize, d.boxes.visible.len());
    assert!(head(&d).grid_moved == 1 && head(&d).items > 0 && d.boxes.grid.items.len() == 2 * head(&d).items as usize);
    assert!(d.boxes.visible.iter().all(|w| (w.w & 0xffff) < head(&d).count), "a drawn box is past the list's end");
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    assert_eq!((head(&d).from, head(&d).to, head(&d).grid_moved, head(&d).fresh), (0, 0, 0, 0), "the same tick drawn again tells nothing anew");
    // (a section asked for alone is made, and no other)
    let mut one = Drawn::default();
    one.take(&p.g, want::lights);
    one.frame(&p.g, 1.0, [0.0; 3]);
    assert!(!one.lights.rows.is_empty() && one.boxes.geo.is_empty() && one.men.men.is_empty());
}
