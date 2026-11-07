//! THE LEVEL'S DOORWAYS, as lines on the floor plan: where a wall has a gap a leaf hangs in (the lights' reach, the picture's places).
use alloc::vec::Vec;
use game::game::Game;

section! {
    /// a doorway: its hinge on the plan, which way the shut leaf lies from it, and how wide it is
    pub struct Doorway {
        pub hinge: [f32; 2],
        pub closed_dir: f32,
        pub width: f32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<Doorway>) {
    for d in &g.level.doors { out.push(Doorway { hinge: d.hinge.map(|x| x as f32), closed_dir: d.closed_dir as f32, width: d.width as f32 }); }
}
