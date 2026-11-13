//! A LEVEL'S LEAVES ARE PAINTED AS THE GAME IS STOOD. A leaf is made in the office's colours whatever the level, and a level whose file says its doors' paint (`Level::door_paint`) has them in that: one that
//! leads outside in the outside's, the rest in the inside's, one in four of those with the outside's paint faded on it, each a shade of its own by its name. Steel doors are left alone, and so is a
//! level that says none. It is the box's own colour and word, so what a breach throws off a leaf and a round's chips take it. No rule asks what colour a leaf is, and the word only says which
//! picture a round's hit has (a leaf is wood to a round whatever its paint: `particle_fx::hit_look`). A leaf keeps what it was stood with.
use crate::game::Game;

/// paint the level's leaves: once, as its doors are made (game_new)
pub fn paint_leaves(g: &mut Game) {
    let Some(paint) = g.level.door_paint else { return };
    let (out, inside) = (paint.outside, paint.inside);
    // (what both are made of, where that is one thing)
    let both = if out.stuff == inside.stuff { out.stuff } else { None };
    let faded = core::array::from_fn(|i| out.albedo[i] * 0.7 + inside.albedo[i] * 0.3);
    for &d in &g.doors.list {
        let (def, leaf) = (&g.dr[d].def, g.dr[d].box_);
        if def.steel == Some(true) { continue; }
        let h = crate::dice::hash_str(&def.name) as u32;
        let k = 0.9 + (h % 1000) as f64 / 1000.0 * 0.2;
        let (albedo, stuff): ([f64; 3], _) = if def.exterior == Some(true) { (out.albedo, out.stuff) } else if (h >> 10) % 4 == 0 { (faded, both) } else { (inside.albedo, inside.stuff) };
        let mut b = g.tab.boxes.edit(leaf);
        b.albedo = albedo.map(|x| x * k);
        b.stuff = stuff;
    }
}
