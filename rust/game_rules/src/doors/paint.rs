//! A LEVEL'S LEAVES ARE PAINTED AS THE GAME IS STOOD (src/scene/leafPaint.ts paintLeaves, called at the same place). A leaf is made in the office's colours whatever
//! the level, and on a mud compound those read as office doors. On a level that WEARS the look an outside door is in the level's own accent paint (its row below, else
//! the one its boxes are painted in, army green where they have none), an inside one in weathered grey wood (one in four with that paint faded on it), each a shade of
//! its own by its name. Steel doors are left alone, and so is a level that carries none of the look's paints. It is the box's own colour and word, so what a breach
//! throws off a leaf and a round's chips take it. No rule asks what colour a leaf is, and the word only says which picture a round's hit has (a leaf is wood to a round
//! whatever its paint: `particle_fx::hit_look`). The look's switch (`k.look.on`, down on an address that says `look=plain`: `scenario_run::address_before_game`) is read here once: a leaf keeps what it was stood with.
use crate::game::Game;
use crate::m;
use crate::scene_gwot_palette::{gwot_palette, snow_palette};
use crate::scene_stuff::BoxStuff;
use alloc::vec::Vec;

type V3 = [f64; 3];
/// what a level's leaves are made of: one word for all, or one a side
#[derive(Clone, Copy)]
enum Said { All(BoxStuff), Sides(Option<BoxStuff>, Option<BoxStuff>), Nothing }
struct Row { out: V3, inside: Option<V3>, stuff: Said }

/// the levels whose accent is not one of the three paints counted off a level's boxes, by the level's own name
fn row_of(level: &str) -> Option<Row> {
    let r = |out: V3, inside: Option<V3>, stuff: Said| Some(Row { out, inside, stuff });
    match level {
        "white-roof" => r([0.88, 0.24, 0.03], Some([0.74, 0.74, 0.73]), Said::All(BoxStuff::Enamel)),
        "block" => r([0.22, 0.4, 0.1], None, Said::Nothing),
        "warehouse" => r([0.6, 0.42, 0.04], None, Said::Sides(Some(BoxStuff::Paint), None)),
        "registry" => r([0.1, 0.1, 0.105], None, Said::Sides(Some(BoxStuff::Paint), None)),
        "records" => r([0.2, 0.2, 0.22], Some([0.47, 0.36, 0.22]), Said::Sides(None, Some(BoxStuff::Wood))),
        "consulate" => r([0.21, 0.05, 0.105], Some([0.27, 0.18, 0.12]), Said::Sides(None, Some(BoxStuff::Wood))),
        "defuse" => r([0.3, 0.14, 0.05], None, Said::Nothing),
        "floodlit" => r([0.62, 0.17, 0.03], None, Said::Nothing),
        "mousehole" => r([0.05, 0.2, 0.19], None, Said::Nothing),
        "killhouse" => r([0.26, 0.12, 0.42], Some([0.55, 0.43, 0.27]), Said::Sides(None, Some(BoxStuff::Wood))),
        "hall" => r([0.16, 0.36, 0.47], None, Said::Nothing),
        _ => None,
    }
}

fn near(a: V3, b: V3) -> bool { (0..3).all(|i| m::abs(a[i] - b[i]) < 0.004) }
/// every colour the look paints a level in
fn look_paints() -> Vec<V3> {
    let (p, s) = (gwot_palette(), snow_palette());
    alloc::vec![p.mud, p.mud_dark, p.mud_light, p.render, p.whitewash, p.block, p.t_wall, p.paint_blue, p.paint_green, p.paint_rust, p.paint_cream, p.dust, p.dirt, p.gravel, p.mud_floor, p.slab, p.damp, p.oil, p.rug_red, p.rug_navy, p.rug_cream, p.rug_ochre, p.rug_green, p.canvas, p.blanket_grey, p.blanket_brown, p.tarp_blue, p.sack, p.sandbag, p.hesco, p.olive, p.coyote, p.stencil, p.rust, p.iron, p.galv, p.tin, p.drum_blue, p.drum_red, p.gen_yellow, p.plastic_white, p.plastic_green, p.plastic_blue, p.plastic_black, p.jerry_yellow, p.rubber, p.poplar, p.wood_grey, p.ply, p.truck_white, p.truck_tan, p.truck_red, p.glass_dark, p.prod_orange, p.prod_green, p.prod_red, p.prod_yellow, p.straw, p.scrub, p.rug_madder, s.snow, s.snow_packed, s.snow_old]
}

/// paint the level's leaves: once, as its doors are made (game_new)
pub fn paint_leaves(g: &mut Game) {
    if !g.k.look.on { return; }
    let (mut count, mut wears) = ([0u32; 3], false);
    let p = gwot_palette();
    let accents = [p.paint_blue, p.paint_green, p.paint_rust];
    let look = look_paints();
    for id in &g.engine.world.statics {
        let a = g.tab.boxes[*id].albedo;
        if !look.iter().any(|r| (0..3).all(|i| m::abs(a[i] - r[i]) <= 0.004)) { continue; }
        wears = true;
        for i in 0..3 { if near(a, accents[i]) { count[i] += 1; } }
    }
    if !wears { return; }
    let name = crate::scene_level_load::level_name_of(g).unwrap_or_default();
    let row = row_of(name.strip_suffix("-gwot").unwrap_or(&name));
    let top = *count.iter().max().unwrap();
    let accent = match &row { Some(r) => r.out, None if top > 0 => accents[count.iter().position(|c| *c == top).unwrap()], None => p.paint_green };
    let wood = row.as_ref().and_then(|r| r.inside).unwrap_or(p.wood_grey);
    let faded: V3 = core::array::from_fn(|i| accent[i] * 0.7 + wood[i] * 0.3);
    let said = |outside: bool| match row.as_ref().map(|r| r.stuff) { Some(Said::Sides(o, i)) => if outside { o } else { i }, _ => None };
    // (a row's colours are numbers, and numbers say nothing; the palette's accents are paint and its grey wood is wood)
    let own = |outside: bool| if outside { row.is_none().then_some(BoxStuff::Paint) } else { row.as_ref().map_or(true, |r| r.inside.is_none()).then_some(BoxStuff::Wood) };
    for &d in &g.doors.list {
        let (def, leaf) = (&g.dr[d].def, g.dr[d].box_);
        if def.steel == Some(true) { continue; }
        let h = crate::math_dice::hash_str(&def.name) as u32;
        let k = 0.9 + (h % 1000) as f64 / 1000.0 * 0.2;
        let outside = def.exterior == Some(true);
        let old = !outside && (h >> 10) % 4 == 0;
        let a = if outside { accent } else if old { faded } else { wood };
        let stuff = if let Some(Row { stuff: Said::All(w), .. }) = row { Some(w) } else if old { None } else { said(outside).or_else(|| own(outside)) };
        let b = &mut g.tab.boxes[leaf];
        b.albedo = a.map(|x| x * k);
        b.stuff = stuff;
    }
}
