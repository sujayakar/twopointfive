//! DOOR LEAVES IN THE LOOK: the picture's alone (nothing of the game asks what colour a leaf is). A leaf is built in the office's colours whatever the level, and on a mud compound those read as office
//! doors: on a level that WEARS the look an outside door is in the level's own accent paint (its row below, else the one its boxes are painted in, army green where they have none), an inside one in
//! weathered grey wood (one in four with that paint faded on it), each a shade of its own by its name. Steel doors are left alone, and so is a level that carries none of the look's paints.
use alloc::vec::Vec;
use game::game::Game;
use game::ids::BoxId;
use game::m;
use game::scene_boxes::Box;
use game::scene_gwot_palette::{gwot_palette, snow_palette};
use game::scene_stuff::BoxStuff;

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
        "block" | "block-squads" => r([0.22, 0.4, 0.1], None, Said::Nothing),
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
/// is this colour one of the look's paints
fn a_paint(a: V3) -> bool {
    let is = |r: &V3| (0..3).all(|i| m::abs(a[i] - r[i]) <= 0.004);
    let (p, s) = (gwot_palette(), snow_palette());
    [p.mud, p.mud_dark, p.mud_light, p.render, p.whitewash, p.block, p.t_wall, p.paint_blue, p.paint_green, p.paint_rust, p.paint_cream, p.dust, p.dirt, p.gravel, p.mud_floor, p.slab, p.damp, p.oil, p.rug_red, p.rug_navy, p.rug_cream, p.rug_ochre, p.rug_green, p.canvas, p.blanket_grey, p.blanket_brown, p.tarp_blue, p.sack, p.sandbag, p.hesco, p.olive, p.coyote, p.stencil, p.rust, p.iron, p.galv, p.tin, p.drum_blue, p.drum_red, p.gen_yellow, p.plastic_white, p.plastic_green, p.plastic_blue, p.plastic_black, p.jerry_yellow, p.rubber, p.poplar, p.wood_grey, p.ply, p.truck_white, p.truck_tan, p.truck_red, p.glass_dark, p.prod_orange, p.prod_green, p.prod_red, p.prod_yellow, p.straw, p.scrub, p.rug_madder, s.snow, s.snow_packed, s.snow_old].iter().any(is)
}

/// what the level's leaves are painted: a leaf's box, its colour and what it is made of. Empty where the look is off or the level does not wear it.
pub fn paints(g: &Game) -> Vec<(BoxId, V3, Option<BoxStuff>)> {
    let mut out = Vec::new();
    if !g.k.look.on { return out; }
    let (mut count, mut wears) = ([0u32; 3], false);
    let p = gwot_palette();
    let accents = [p.paint_blue, p.paint_green, p.paint_rust];
    for id in &g.engine.world.statics {
        let a = g.tab.boxes[*id].albedo;
        if !a_paint(a) { continue; }
        wears = true;
        for i in 0..3 { if near(a, accents[i]) { count[i] += 1; } }
    }
    if !wears { return out; }
    let name = game::scene_level_load::level_name_of(g).unwrap_or_default();
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
        let h = game::math_dice::hash_str(&def.name) as u32;
        let k = 0.9 + (h % 1000) as f64 / 1000.0 * 0.2;
        let outside = def.exterior == Some(true);
        let old = !outside && (h >> 10) % 4 == 0;
        let a = if outside { accent } else if old { faded } else { wood };
        let stuff = if let Some(Row { stuff: Said::All(w), .. }) = row { Some(w) } else if old { None } else { said(outside).or_else(|| own(outside)) };
        out.push((leaf, a.map(|x| x * k), stuff));
    }
    out.sort_by_key(|x| x.0 .0);
    out
}
/// a leaf in its paint
pub fn on(b: &Box, a: V3, stuff: Option<BoxStuff>) -> Box { Box { albedo: a, stuff, ..b.clone() } }
